# Learning Rust - Axum

Web API with Rust.  
**Axum** library for API service.  
**Sqlx** library as database helper.  

[![Deploy](https://github.com/alex-piccione/learning.Rust.Axum/actions/workflows/deploy.yml/badge.svg)](https://github.com/alex-piccione/learning.Rust.Axum/actions/workflows/deploy.yml)

## Setup e SQLx

See [SQLx](src/repositories/SQLx.md).


## Development

_rust-analyzer_ continuosly check the code, but `cargo build` will compile the project with a final list of errors.    
SQLx is set to verify the database entity and need to be able to access the database.  
It uses the **DATABASE_URL** variable in the _.env_ file, it points t oa local Docker contyaginer with Postgres.  
`cargo sqlx prepare`
  
Rust standard library does not have Date or Datetime types (!).  
A modern approach is to use _chrono_ but also _sqlx_ offer it.  
To keep it simple, I'll use sqlx types. 
I'll use _OffsetDateTime_ but always as UTC.  

## Run locally

VS Code launch (_launch.json_) is set to run the app locally.  


## Deploy

### On Azure

TODO: is this still true ??

API is deployed in Azure, in a Web App Service.  
[TODO: detail CD on Azure with GitHiub action]  
Since the database is not part of this deploy, it is not described here.


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
_static mut_ is Rust is ... not possible.  
Not without _unsafe_.  
In other languages you have thread-safe colelctions, but not in Rust standard library.  
You can paly around and write cumbersome code with LazyLock, RwLock and Mutex... that's it.  
Otherwise, third party library. I coose "Dashmap". 
