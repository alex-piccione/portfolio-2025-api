delete from CurrencyRates where "date"::time <> '00:00:00';

ALTER TABLE CurrencyRates
ALTER COLUMN "date" TYPE DATE;
