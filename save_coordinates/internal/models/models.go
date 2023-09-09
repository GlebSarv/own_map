// Import the necessary package.
package models

import "time"

// Define a struct named GeoData to represent geographical data.
type GeoData struct {
	Latitude  float64   `json:"lat"`      // Latitude in degrees
	Longitude float64   `json:"long"`     // Longitude in degrees
	Altitude  float64   `json:"altitude"` // Altitude in meters
	Tmstmp    time.Time `json:"tmstmp"`   // Timestamp associated with the data
}

// NewGeoData is a constructor function for creating a new GeoData instance.
// Parameters:
// - latitude: A float64 representing the latitude coordinate.
// - longitude: A float64 representing the longitude coordinate.
// - altitude: A float64 representing the altitude coordinate.
// - tmstmp: A time.Time representing the timestamp associated with the GeoData.
// Returns:
// - A pointer to a new GeoData instance initialized with the specified values.
func NewGeoData(latitude, longitude, altitude float64, tmstmp time.Time) *GeoData {
	// Create and return a new GeoData instance with the specified values.
	return &GeoData{
		Latitude:  latitude,
		Longitude: longitude,
		Altitude:  altitude,
		Tmstmp:    tmstmp,
	}
}
