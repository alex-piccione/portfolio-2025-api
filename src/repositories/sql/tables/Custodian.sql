CREATE TABLE Custodian (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    kind VARCHAR(25) NOT NULL CHECK (kind IN ('Bank', 'Exchange', 'Pension', 'Blockchain Wallet')),
    description TEXT,
    url VARCHAR(255),
    wallet_address VARCHAR(255),
    country_code VARCHAR(2), -- ISO 3166-1 alpha-2 code like 'IT' or 'US'
);