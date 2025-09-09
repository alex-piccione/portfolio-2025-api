-- Remove the CHECK constraint from the KIND column in the Currency table
ALTER TABLE Currency DROP CONSTRAINT currency_kind_check;

-- Remove the CHECK constraint from the KIND column in the Custodian table
ALTER TABLE Custodian DROP CONSTRAINT custodian_kind_check;

-- Rename the country_code column to account_country_code in Custodian table
ALTER TABLE Custodian RENAME COLUMN country_code TO account_country_code;