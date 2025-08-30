# Devop

## Database

Database is a Postgres docker container running on a Linux VPS.  
I'm using [SQLx](https://docs.rs/sqlx/latest/sqlx) to manage database interaction.  


### Setup

Instead of a script that call `docker run` I will use a docker-compose.  
In this way if in the future we need to add some other service it will be easy.  

Run ``docker compose up`` from this folder,   
it will use the _.env_ file (git-ignored) to get the environment variables for secrets.  


### Secrets/Configuration

Configuration is loadd by a _configuration.json_ file.  
For local development we have a gitignored file in the solution,  
for remote environment a **CONFIGURATION_FILE** environment variable should indicates where to read that file.


### Deploy


### Launch

## Local

_launch.js_ of Visual Studio code has a command to launch the app.  
A local configuration file must exists.  

## Remote

