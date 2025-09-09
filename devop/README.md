# Devop

## Database

Database is a Postgres docker container running on a Linux VPS.  
I'm using [SQLx](https://docs.rs/sqlx/latest/sqlx) to manage database interaction.  
We use SQLx macros that check the SQL over the database, see the SQLx paragraph.  


### Setup Local database

Instead of a script that call `docker run` I will use a docker-compose.  
In this way if in the future we need to add some other service it will be easy.  

Run ``docker compose up`` from this folder,   
it will use the _.env_ file (git-ignored) to get the environment variables for secrets.  

### Connection string

The database connection string is set in the Configuration.  
The Configuration is filled with a _configuration.json_ file.  
For local development we have a git-ignored file in the solution,  
for remote environment a **CONFIGURATION_FILE** environment variable should indicates where to read that file.  

## SQLx

SQLx-CLI needs to be installed: ``cargo install sqlx-cli --no-default-features --features "postgres"``
  
These commands can be used to manage database creation and changes:
- ``cargo sqlx prepare``         # Generate query metadata (local cache used by static analizer)
- ``cargo sqlx migrate run``     # Run database migrations
- ``cargo sqlx database create`` # Create database
- ``cargo sqlx database drop``   # Drop database

See the *.local_sqlx_comamnds.sh* file with prepared commands.  

The application, at start, will check and execute the migrations if configuration enabled it.  
 


### Server database

It is not part of this project.



## Known Issues

- Debug warning about LLDB not able to debug
  > The LLDB warnings about missing Rust plugins are normal on Windows and do not affect your app's runtime, but they limit debugging features.

  

