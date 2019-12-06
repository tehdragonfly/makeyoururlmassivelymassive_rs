-- Your SQL goes here

CREATE TABLE massive_urls (
    id SERIAL PRIMARY KEY,
    path VARCHAR NOT NULL,
    destination VARCHAR NOT NULL
);
