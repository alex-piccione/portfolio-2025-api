/*
ADD CONSTRAINT users_username_key UNIQUE (username); 
// Posthgres automatically add a unique index for unique constraint but.. let's use a more explicit command

Postgres use the general collation of the database, usually en_US.UTF-8 (if a COLLATE is not specified)
or use its specific CITEXT functionality
*/

/* CREATE UNIQUE INDEX users_username_idx ON users(LOWER(username)); */


-- enable the citext extension (only needs to run once per database)
CREATE EXTENSION IF NOT EXISTS citext;

-- change column type
ALTER TABLE users
ALTER COLUMN username TYPE citext;

-- add unique constraint
ALTER TABLE users
ADD CONSTRAINT users_username_unique UNIQUE (username);