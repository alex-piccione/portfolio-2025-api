CREATE TABLE CurrencyRates (
    base_currency_id INT NOT NULL,
    quote_currency_id INT NOT NULL,
    "date" TIMESTAMPTZ NOT NULL,
    source VARCHAR(50) NOT NULL,
    rate NUMERIC(38, 18) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (base_currency_id, quote_currency_id, date, source),
    CONSTRAINT CurrencyRates_fk_base_currency 
        FOREIGN KEY (base_currency_id) REFERENCES currency(id) ON DELETE RESTRICT,
    CONSTRAINT CurrencyRates_fk_quote_currency 
        FOREIGN KEY (quote_currency_id) REFERENCES currency(id) ON DELETE RESTRICT
);

CREATE INDEX CurrencyRates_idx_date_source ON CurrencyRates(date);