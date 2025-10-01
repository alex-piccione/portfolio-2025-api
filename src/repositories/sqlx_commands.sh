## Create the tables

cargo sqlx migrate add create_currency_table
# it will create a file in migrations folder, edit it to add the SQL code to create the tables (from src/repositories/sql/tables)

cargo sqlx migrate run
# will use the migration file to create the tables in the database


cargo sqlx database create
# what is this for ?


cargo sqlx prepare
# will create a database cache so that the SQLx macros can validate the queries at compile time
# it will create a file sqlx-data.json in the root folder
# if you change the database schema, you need to run this command again to update the cache


# create the Custodian table migration
cargo sqlx migrate add create_custodian_table
# copy thre SQL code...

# execute the migration
cargo sqlx migrate run


## Update a table
cargo sqlx migrate add custodian_currency_updates

# Add hte SQL comnmands there and execute 
cargo sqlx migrate run


## Add User entity
cargo sqlx migrate add create_user_table