-- delete old FK (old custodian table)
ALTER TABLE Holdings
DROP CONSTRAINT holdings_custodian_fk;

-- Delete all records
delete from Holdings;

-- add new FK (new custodians table)
ALTER TABLE Holdings
ADD CONSTRAINT holdings_custodian_fk FOREIGN KEY (custodian_id) REFERENCES custodians(id) ON DELETE CASCADE;

-- remove old table
drop table custodian;