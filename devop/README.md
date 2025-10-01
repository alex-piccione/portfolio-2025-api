# Devop

## Database

The database is a Postgres docker container running on a Linux VPS, not part of this project.    
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
 
SQLx use the **DATABASE_URL** environment variable to read the connection string.  
See the _.env_ file.


## Test Docker image locally

See _local_Dockerfile.sh_.


## Deploy

Deploy is executed with a GitHub action that launch a script on a private server.  
The script execution is allowed by a SSH Restricted permission key.  
The description and the procedure is not part of this project.  

### Investigate deploy failure

When thge deploy script fails it is possible to look at the Docker container log to see why it fails to start.  
```sh
# get the id of the failed stack
docker stack ps portfolio-api
stack_id=$(docker stack ps portfolio-api --format "{{.ID}} {{.CurrentState}} {{.Error}}" | grep Failed | awk '{print $1}')
echo "stack_id=$stack_id"

container_id=$(docker inspect $stack_id --format '{{.Status.ContainerStatus.ContainerID}}')
echo "container_id=$container_id"

docker logs $container_id

#docker inspect -it $container_id sh
```


## Known Issues

- Debug warning about LLDB not able to debug
  > The LLDB warnings about missing Rust plugins are normal on Windows and do not affect your app's runtime, but they limit debugging features.

  

