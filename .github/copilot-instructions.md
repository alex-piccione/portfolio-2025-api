# API for an application to manage owned assets

## Tech stack
- Rust
- Axum library
- SQLx for database interaction
- Serde for serialization/deserialization
- Tokio for async runtime     
- PostgreSQL database   
- Docker for containerization
- GitHub Actions for CI/CD


## Project structure

The src folder contains the main application code, organized into modules for better maintainability. The main.rs file initializes the application and sets up the server.
The Cargo.toml file manages dependencies and project metadata.

src/
  ├── main.rs         # Entry point of the application
  ├── endpoints       # API endpoints
  │   └── models      # Endpoint-specific data models  
  ├── entities        # Data models
  ├── services        # Business logic
  ├── repositories    # Database interaction
  │   └── schemas     # Dataabse records mapping
  ├── migrations      # database SQL commands for updates
  └── utils           # Utility functions and helpers          
      └── routing.rs  # Routing definiktion and utilities


