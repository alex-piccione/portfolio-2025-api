-- Step 1: add the column with a temporary default
ALTER TABLE users
ADD COLUMN role VARCHAR(100) NOT NULL DEFAULT 'User';

-- Step 2: remove the default so new rows must provide a value
ALTER TABLE users
ALTER COLUMN role DROP DEFAULT;