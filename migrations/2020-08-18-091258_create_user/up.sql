-- Your SQL goes here
CREATE TABLE users (
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    id VARCHAR PRIMARY KEY NOT NULL,
    address TEXT NOT NULL,
    password VARCHAR NOT NULL,
    birthdate VARCHAR NOT NULL
)