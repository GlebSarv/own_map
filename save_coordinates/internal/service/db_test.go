package service

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"os"
	"testing"
	"time"

	"github.com/glebSarv/save_coordinates/internal/database"
	_ "github.com/lib/pq"
	"github.com/stretchr/testify/require"
)

var testQueries *database.Queries

func TestMain(m *testing.M) {

	connString := "postgresql://postgres:postgres@localhost:5433/own_map?sslmode=disable"
	conn, err := sql.Open("postgres", connString)
	if err != nil {
		log.Fatalf("Failed to connect to the database: %v", err)
	}
	defer conn.Close()

	testQueries = database.New(conn)

	exitCode := m.Run()

	os.Exit(exitCode)
}

func TestAddCoordinates(t *testing.T) {
	latitude := 59.849815368652344
	longitude := 30.321691513061523
	altitude := 53.83399963378906
	tmstmp := time.Now()
	arg := database.AddCoordinatesParams{
		Latitude:  latitude,
		Longitude: longitude,
		Altitude:  altitude,
		Tmstmp:    tmstmp,
	}

	geoData, err := testQueries.AddCoordinates(context.Background(), arg)
	if err != nil {
		fmt.Println(geoData)
	}

	require.NoError(t, err)
	require.NotEmpty(t, geoData)

	require.Equal(t, latitude, geoData.Latitude)
	require.Equal(t, longitude, geoData.Longitude)
	require.Equal(t, altitude, geoData.Altitude)

	require.NotZero(t, geoData.ID)
	require.NotZero(t, geoData.Tmstmp)
}
