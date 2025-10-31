CREATE TABLE Custodians (
    id SERIAL PRIMARY KEY,    
    user_id CHAR(36) NOT NULL,
    name VARCHAR(25) NOT NULL,
    custodian VARCHAR(50) NOT NULL,
    account VARCHAR(50) NULL,
    kind VARCHAR(25) NOT NULL,
    color_code VARCHAR(15) NOT NULL,
    description TEXT
);

-- Foreign key to users
ALTER TABLE Custodians
ADD CONSTRAINT custodians_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- Index for quick lookups by user_id
CREATE INDEX custodians_user_id_idx ON Custodians (user_id);

-- Unique constraints
ALTER TABLE Custodians
ADD CONSTRAINT custodians_user_id_name_unique UNIQUE (user_id, name);


ALTER TABLE Custodians
ADD CONSTRAINT custodians_custodian_account_unique UNIQUE (custodian, account);