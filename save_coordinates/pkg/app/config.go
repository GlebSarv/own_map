// Import necessary packages and modules.
package app

import (
	"log"
	"os"
	"strconv"

	"github.com/joho/godotenv"
)

// Define a configuration structure that holds various configuration settings.
type Config struct {
	Consumer *KafkaConsumerConfig // Kafka consumer configuration
	PG       *PGConfig            // PostgreSQL database configuration
	Grpc     *GrpcConfig          // gRPC server configuration
}

// Define a structure for Kafka consumer configuration.
type KafkaConsumerConfig struct {
	BootstrapServer string // Kafka bootstrap server address
	Port            int    // Kafka port
	GroupId         string // Kafka consumer group ID
	Topic           string // Kafka topic to consume
}

// Define a structure for PostgreSQL database configuration.
type PGConfig struct {
	Port     int    // PostgreSQL database port
	Host     string // PostgreSQL database host
	Database string // PostgreSQL database name
	User     string // PostgreSQL database user
	Password string // PostgreSQL database password
	Driver   string // PostgreSQL database driver
}

// Define a structure for gRPC server configuration.
type GrpcConfig struct {
	Host string // gRPC server host address
	Port int    // gRPC server port
}

// NewConfig creates a new configuration by reading environment variables using godotenv.
func NewConfig() (*Config, error) {
	// Load environment variables from a .env file
	if err := godotenv.Load(); err != nil {
		log.Fatalf("Error on loading file %v", err)
		return nil, err
	}

	// Read Kafka consumer configuration from environment variables
	bootstrapServers := os.Getenv("CONSUMER_BOOTSTRAP_SERVER")
	consumerPort, err := strconv.Atoi(os.Getenv("CONSUMER_PORT"))
	if err != nil {
		return nil, err
	}
	groupID := os.Getenv("CONSUMER_GROUPID")
	topic := os.Getenv("CONSUMER_TOPIC")

	// Read PostgreSQL configuration from environment variables
	pgHost := os.Getenv("PG_HOST")
	pgPort, err := strconv.Atoi(os.Getenv("PG_PORT"))
	if err != nil {
		return nil, err
	}
	pgUser := os.Getenv("PG_USER")
	pgPassword := os.Getenv("PG_PASSWORD")
	pgDatabase := os.Getenv("PG_DATABASE")
	pgDriver := os.Getenv("PG_DRIVER")

	// Read gRPC server configuration from environment variables
	grpcHost := os.Getenv("GRPC_HOST")
	grpcPort, err := strconv.Atoi(os.Getenv("GRPC_PORT"))
	if err != nil {
		return nil, err
	}

	// Create and return a configuration struct with the parsed values
	return &Config{
		Consumer: &KafkaConsumerConfig{
			BootstrapServer: bootstrapServers,
			Port:            int(consumerPort),
			GroupId:         groupID,
			Topic:           topic,
		},
		PG: &PGConfig{
			Host:     pgHost,
			Port:     pgPort,
			User:     pgUser,
			Password: pgPassword,
			Database: pgDatabase,
			Driver:   pgDriver,
		},
		Grpc: &GrpcConfig{
			Host: grpcHost,
			Port: grpcPort,
		},
	}, nil
}
