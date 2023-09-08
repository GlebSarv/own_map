-- +goose Up
CREATE TABLE IF NOT EXISTS geo_data (
    id serial primary key,
    latitude float not null,
    longitude float not null,
    altitude float not null,
    tmstmp TIMESTAMP not null
);

-- +goose Down
DROP TABLE geo_data;