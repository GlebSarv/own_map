// Import necessary packages and modules.
package app

import (
	"database/sql"
	"fmt"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"github.com/glebSarv/save_coordinates/internal/database"
	"github.com/glebSarv/save_coordinates/internal/service"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	_ "github.com/lib/pq"
)

// Define the Pgx struct to hold postgres components.
type Pgx struct {
	Conn    *sql.DB
	Queries service.Storage
}

// Define the App struct to hold application components.
type App struct {
	Consumer   *kafka.Consumer
	GrpcClient *grpc.ClientConn
	Pg         *Pgx
	KafkaTopic string
}

// NewApp creates a new instance of the App struct and initializes its components.
func NewApp() (*App, error) {

	a := &App{}
	config, err := NewConfig()
	if err != nil {
		return nil, err
	}

	a.Consumer, err = newConsumer(config.Consumer)

	if err != nil {
		return nil, err
	}
	a.GrpcClient, err = newGrpcClient(config.Grpc)

	if err != nil {
		return nil, err
	}

	a.Pg, err = newStore(config.PG)

	if err != nil {
		return nil, err
	}
	a.KafkaTopic = config.Consumer.Topic

	return a, nil
}

// Define the newConsumer function to create a Kafka consumer.
func newConsumer(config *KafkaConsumerConfig) (*kafka.Consumer, error) {
	server := fmt.Sprintf("%s:%d", config.BootstrapServer, config.Port)

	consumer, err := kafka.NewConsumer(
		&kafka.ConfigMap{
			"bootstrap.servers": server,
			"group.id":          config.GroupId,
			"auto.offset.reset": "smallest",
		},
	)

	if err != nil {
		return nil, err
	}

	return consumer, nil
}

// Define the newStore function to create a PostgreSQL storage connection.
func newStore(config *PGConfig) (*Pgx, error) {

	connString := fmt.Sprintf(
		"%s://%s:%s@%s:%d/%s?sslmode=disable",
		config.Driver,
		config.User,
		config.Password,
		config.Host,
		config.Port,
		config.Database,
	)

	conn, err := sql.Open("postgres", connString)

	if err != nil {
		return nil, err
	}

	apiQueries := service.ApiCfg{Q: database.New(conn)}

	return &Pgx{Conn: conn, Queries: &apiQueries}, nil
}

// Define the newGrpcClient function to create a gRPC client connection.
func newGrpcClient(config *GrpcConfig) (*grpc.ClientConn, error) {
	var opts []grpc.DialOption
	opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))
	address := fmt.Sprintf("%s:%d", config.Host, config.Port)

	conn, err := grpc.Dial(address, opts...)

	if err != nil {
		return nil, err
	}

	return conn, nil
}
