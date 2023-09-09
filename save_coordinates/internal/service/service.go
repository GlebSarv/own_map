package service

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"sync"

	"github.com/confluentinc/confluent-kafka-go/kafka"
	"github.com/glebSarv/save_coordinates/internal/database"
	"github.com/glebSarv/save_coordinates/internal/models"
)

// Define a Storage interface for database operations.
type Storage interface {
	RecordCoordinates(conn *sql.DB, ch chan models.GeoData, wg *sync.WaitGroup) error
}

// ApiCfg is a struct that holds database queries.
type ApiCfg struct {
	Q *database.Queries
}

// RecordCoordinates is a method that records GeoData into a PostgreSQL database.
// Parameters:
// - conn: A pointer to a SQL database connection (*sql.DB) for executing SQL queries.
// - ch: A channel of type models.GeoData used to receive GeoData to be recorded in the database.
// - wg: A WaitGroup that helps coordinate goroutines; it is used for signaling when this method completes.
// Returns:
// - An error if any database operation encounters an error; otherwise, it returns nil upon successful completion.
func (api *ApiCfg) RecordCoordinates(conn *sql.DB, ch chan models.GeoData, wg *sync.WaitGroup) error {
	// Defer decreasing the WaitGroup counter and closing the database connection when this function exits.
	defer wg.Done()
	defer func() {
		fmt.Println("Close db connection")
		conn.Close()
	}()

	// Iterate over messages received from the channel and insert them into the database.
	for msg := range ch {
		// Create an argument (database.AddCoordinatesParams) for the database query.
		arg := database.AddCoordinatesParams{
			Latitude:  msg.Latitude,
			Longitude: msg.Longitude,
			Altitude:  msg.Altitude,
			Tmstmp:    msg.Tmstmp,
		}

		// Call the AddCoordinates query to insert data into the database.
		res, err := api.Q.AddCoordinates(
			context.Background(),
			arg,
		)

		// Print the result (res) and return an error if any database operation fails.
		fmt.Println("res", res)
		if err != nil {
			return err
		}
	}

	// Return nil to indicate successful completion of recording GeoData.
	return nil
}

// Consume is a function that consumes Kafka messages and sends GeoData to a channel.
// Parameters:
// - consumer: A pointer to a Kafka consumer instance (*kafka.Consumer) used for receiving Kafka messages.
// - topic: A string representing the name of the Kafka topic to subscribe to.
// - ch: A channel of type models.GeoData used to pass deserialized GeoData to the main program.
// - wg: A WaitGroup that helps coordinate goroutines; it is used for signaling when this function completes.
func Consume(consumer *kafka.Consumer, topic string, ch chan models.GeoData, wg *sync.WaitGroup) {
	// Defer decreasing the WaitGroup counter and closing the Kafka consumer and logging its closure.
	defer wg.Done()
	defer func() {
		consumer.Close()
		log.Println("Kafka consumer closed")
	}()

	// Subscribe to the specified Kafka topic.
	err := consumer.Subscribe(topic, nil)
	if err != nil {
		fmt.Println(err)
	}

	for {
		select {
		// This case handles the scenario when the main program signals to stop the consumer.
		case <-ch:
			log.Println("Stopping Kafka consumer")
			return

		// This default case processes Kafka events.
		default:
			// Poll for Kafka events with a timeout of 100 milliseconds.
			event := consumer.Poll(100)

			// If there is no event, continue to the next iteration.
			if event == nil {
				continue
			}

			// Check the type of event.
			switch e := event.(type) {
			case *kafka.Message:
				// Create an empty GeoData struct to store the deserialized data.
				message := models.GeoData{}

				// Deserialize the Kafka message payload (e.Value) into the GeoData struct.
				if err := json.Unmarshal(e.Value, &message); err != nil {
					log.Printf("error  %v\n", err)
				}

				// Send the deserialized GeoData to the channel for further processing.
				ch <- message

			case *kafka.Error:
				// Handle Kafka errors (e.g., connection issues, topic errors) if any.
				log.Printf("error  %v\n", e)
			}
		}
	}
}


