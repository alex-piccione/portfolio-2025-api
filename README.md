# Learning Rust - Axum

[![Deploy](https://github.com/alex-piccione/learning.Rust.Axum/actions/workflows/deploy.yml/badge.svg)](https://github.com/alex-piccione/learning.Rust.Axum/actions/workflows/deploy.yml)

Web API with Rust.  
**Axum**: library for API service.  
**Sqlx**: library to interact with database (compile-time scghema cheks).


## Setup

- _.env_ file: create a .env file on the root, see _.env_example_ as reference. 
- convert migrations file to Linux format, if on Windows machine ( see README inside /migrations folder) 


## Development

_rust-analyzer_ continuosly check the code and highlight issues, `cargo build` will compile the project with the list of errors too.    

### SQLx

SQLx is set to verify the database entity and need to be able to access the database.  
It uses the **DATABASE_URL** variable set in the _.env_ file, it points to a local Docker container with Postgres.  
`cargo sqlx prepare`.  
  
Refer to thte [SQLx](src/repositories/SQLx.md) readme.

### DateTime
Rust standard library does not have Date or Datetime types (!).  
A modern approach is to use _chrono_ but also _sqlx_ offer it.  
To keep it simple, I'll use sqlx types. 
I'll use _OffsetDateTime_ of SQLx (always as UTC).  
I created UtcDateTime custom type... I tried to use only std types like Duration... rubbish!  
In the end only _chrono_ has a clear and short "Utc::now()" function and allows "<", ">" and "=" operators.  

## Run locally

### Localhost
VS Code launch (_launch.json_) is set to run the app locally.  

Sometime the process is still running despite terminal and debug areclosed.  
Run this to find teh process PID (<process>.exe):  
```sh
tasklist | findstr portfolio`
```
Run this to delete it:  
```powershell or CMD
taskkill /PID <PID> /F
```

### Docker
See [devop/README.md] for instruciton to run the api and database on local Docker.  

## Deploy

### On private server

"distroless" Dockerfile
✅ Minimal attack surface (no shell, package manager, etc.)
✅ Only contains your application and minimal runtime
✅ Regularly updated by Google
✅ Industry standard for production containers

The deploy is executed running a script on a private server.  
See [devop/README.md](devop/README.md#Deploy) for how to configure the script.



## Tips

### Read environment variable

```rust
// read the port from environment variable or use a default
let port = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse::<u16>()
    .expect("Failed to parse PORT environment variable as a number");
```

### static mut
_static mut_ in Rust is ... not possible.  
Not without _unsafe_.  
In other languages you have thread-safe colelctions, but not in Rust standard library.  
You can paly around and write cumbersome code with LazyLock, RwLock and Mutex... that's it.  
Otherwise, third party library. I coose "Dashmap". 

### Read local file
(used previously to read configuration)

```rust
let config_file = match std::fs::exists(CONFIGURATION_FILE) {
    Ok(true) => { 
        println!("Using configuration file '{}'.", CONFIGURATION_FILE); 
        String::from(CONFIGURATION_FILE)
    },
    Ok(false) => { 
        println!("Configuration file '{}' not found, using CONFIGURATION_FILE environment variable.", CONFIGURATION_FILE); 

    },
    Err(e) => panic!("Failed to check for local configuration file '{}': {}", CONFIGURATION_FILE, e),
};
```
