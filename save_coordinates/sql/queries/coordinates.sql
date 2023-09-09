-- name: AddCoordinates :one
INSERT INTO geo_data (latitude, longitude, altitude, tmstmp)
VALUES($1, $2, $3, $4)
RETURNING *;
