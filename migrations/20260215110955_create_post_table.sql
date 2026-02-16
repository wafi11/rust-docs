-- Add migration script here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    description VARCHAR(200)
);

