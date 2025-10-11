# SQLx

SQLx-CLI needs to be installed: ``cargo install sqlx-cli --no-default-features --features "postgres"``

SQLx use the **DATABASE_URL** environment variable to read the connection string. 
There is an _.env_ file in the root folder, so SQLx commands **MUST be run from the root folder**,
or: `export DATABASE_URL="postgresql://username:password@localhost/database_name"`

The application, at start, will check and execute the migrations if configuration enabled it.  

These commands can be used to manage database creation and changes:
- ``cargo sqlx prepare``         # Generate query metadata (local cache used by static analizer)
- ``cargo sqlx migrate run``     # Run database migrations
- ``cargo sqlx database create`` # Create database
- ``cargo sqlx database drop``   # Drop database

See the *sqlx_comamnds.sh* file with prepared commands.  


```sql
select * from _sqlx_migrations  [TBC]
```


## Add a new entity

1. Create the entitty in /entities
2. ``cargo sqlx migrate add create_<entity>_table``
   it will create an empty file in /migrations folder
3. create a valid SQL command using a real database,
   once done, delete the table and update the create_<entity>_table
4. ``cargo sqlx migrate run`` will execute the migration
