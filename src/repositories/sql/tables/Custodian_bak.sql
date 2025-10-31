CREATE TABLE Custodians (
    id SERIAL PRIMARY KEY,    
    user_id CHAR(36) NOT NULL,
    short_name VARCHAR(25) NOT NULL UNIQUE,
    ADD COLUMN custodian VARCHAR(25),
    ADD COLUMN account VARCHAR(25) NULL,
    kind VARCHAR(25) NOT NULL,
    description TEXT,
    url VARCHAR(255),
    wallet_address VARCHAR(255),
    account_country_code CHAR(2) NULL, -- ISO 3166-1 alpha-2 code like 'IT' or 'US'
);

-- Foreign key to users
ALTER TABLE Sessions
ADD CONSTRAINT custodians_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- Index for quick lookups by user_id
CREATE INDEX sessions_user_id_idx ON Sessions (user_id);

-- Unique constraints
ALTER TABLE Sessions
ADD CONSTRAINT sessions_access_token_unique UNIQUE (access_token);