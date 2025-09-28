# Learning Rust - Axum

Web API with Rust.  
**Axum** library for API service.  
**Sqlx** library as database helper.  


## Setup e SQLx

See [devop/README.md](devop/README.md).


## Development

_rust-analyzer_ continuosly check the code, but `cargo build` will compile the project.  
SQLx is set to verify the database entity and need to be able to access the database.  

`cargo sqlx prepare`


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