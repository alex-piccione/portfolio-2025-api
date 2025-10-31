CREATE TABLE Holdings (
    id SERIAL PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    custodian_id int NOT NULL,
    currency_id int NOT NULL,
    "date" TIMESTAMPTZ NOT NULL,
    "action" VARCHAR(50) NOT NULL,
    amount DECIMAL(27,18),
    note VARCHAR(500)
);

-- Foreign keys
ALTER TABLE Holdings
ADD CONSTRAINT holdings_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE Holdings
ADD CONSTRAINT holdings_custodian_fk FOREIGN KEY (custodian_id) REFERENCES custodians(id) ON DELETE CASCADE;

ALTER TABLE Holdings
ADD CONSTRAINT holdings_currency_fk FOREIGN KEY (currency_id) REFERENCES currency(id) ON DELETE CASCADE;

-- Index for quick lookups by user_id
CREATE INDEX holdings_user_id_date_idx ON Holdings (user_id, "date");

-- CREATE INDEX holdings_custodian_idx ON Holdings (custodian_id);
-- CREATE INDEX holdings_currency_idx ON Holdings (currency_id);