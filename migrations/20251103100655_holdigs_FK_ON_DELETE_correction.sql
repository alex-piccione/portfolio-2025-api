-- Change foreign key on Custodian and Currency to avoid CASCADE delete (set RESTRICT)

ALTER TABLE Holdings
DROP CONSTRAINT holdings_custodian_fk;

ALTER TABLE Holdings
ADD CONSTRAINT holdings_custodian_fk FOREIGN KEY (custodian_id) REFERENCES custodians(id) ON DELETE RESTRICT;



ALTER TABLE Holdings
DROP CONSTRAINT holdings_currency_fk;

ALTER TABLE Holdings
ADD CONSTRAINT holdings_currency_fk FOREIGN KEY (currency_id) REFERENCES currency(id) ON DELETE RESTRICT;