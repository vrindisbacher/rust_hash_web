-- Your SQL goes here
CREATE TABLE Auth (
    id VARCHAR NOT NULL PRIMARY KEY,
    salt VARCHAR NOT NULL, 
    hashed_value VARCHAR NOT NULL,
    encryption_algorithm VARCHAR NOT NULL
)