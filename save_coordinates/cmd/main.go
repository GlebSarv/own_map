package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"sync"

	// Importing necessary packages and modules
	"github.com/glebSarv/save_coordinates/internal/models"
	"github.com/glebSarv/save_coordinates/internal/service"
	"github.com/glebSarv/save_coordinates/pkg/app"
	pb "github.com/glebSarv/save_coordinates/proto"
)

func main() {
	// Create a new application instance using the app package
	app, err := app.NewApp()
	if err != nil {
		log.Fatalf("error while creating application: %v", err)
		os.Exit(1)
	}

	// Create a gRPC client to communicate with a remote service
	c := pb.NewExifReadersClient(app.GrpcClient)

	// Create a context for the gRPC request
	ctx := context.Background()

	// Make a gRPC request to a remote service (WalkingDirectory)
	response, err := c.WalkingDirectory(ctx, &pb.ExifReaderRequest{DirectoryName: "/home/gleb/Desktop/test"})
	if err != nil {
		log.Fatalf("could not walk directory: %v", err)
	}

	// Print the response received from the remote service
	fmt.Println(response)

	// Create a channel for passing GeoData models
	messageChannel := make(chan models.GeoData)

	// Create a WaitGroup to wait for goroutines to finish
	var wg sync.WaitGroup

	// Start a goroutine to consume Kafka messages
	wg.Add(1)
	go service.Consume(app.Consumer, app.KafkaTopic, messageChannel, &wg)

	// Start a goroutine to record coordinates in PostgreSQL
	wg.Add(1)
	go app.Pg.Queries.RecordCoordinates(app.Pg.Conn, messageChannel, &wg)

	// Set up a signal channel to handle interruptions (e.g., Ctrl+C)
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt)
	<-sigChan // Wait for an interrupt signal

	// Close the message channel to signal the consumer and writer to stop
	close(messageChannel)

	// Wait for the goroutines to finish processing
	wg.Wait()

	// Application has gracefully shut down
}
