CREATE TABLE Users (
    id CHAR(36) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    hashed_password VARCHAR NOT NULL,
    currency_id INTEGER NOT NULL REFERENCES Currency(id),
    creation_date TIMESTAMPTZ NOT NULL    
);

/* VARCHAR(255) or VARCHAR are managed the same, also in terms of performance, but we try to prevent the possibility that someone submit a username of 10MB. */
