CREATE TABLE CurrenciesOfUser (
    id SERIAL PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    currency_id int NOT NULL
);

-- Foreign keys
ALTER TABLE CurrenciesOfUser
ADD CONSTRAINT CurrenciesOfUser_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE CurrenciesOfUser
ADD CONSTRAINT CurrenciesOfUser_currency_fk FOREIGN KEY (currency_id) REFERENCES currency(id) ON DELETE CASCADE;

-- Index for quick lookups by user_id
CREATE INDEX CurrenciesOfUser_user_id_currency_id_idx ON CurrenciesOfUser (user_id, currency_id);
