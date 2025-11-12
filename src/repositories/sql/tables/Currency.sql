CREATE TABLE Currency (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL UNIQUE,
    kind VARCHAR(20) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    precision SMALLINT NOT NULL CHECK (precision >= 0 AND precision <= 18),
    coingecko_id varchar(15) NULL,
);
