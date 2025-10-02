CREATE TABLE Users (
    id CHAR(36) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    hashed_password VARCHAR NOT NULL,
    currency_id INTEGER NOT NULL REFERENCES Currency(id),
    creation_date TIMESTAMPTZ NOT NULL    
);