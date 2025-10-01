use axum::{
    http::{self, HeaderValue}, routing::{get, post, put}, Router
};
use tower_http::cors::{CorsLayer, Any};
use tokio::{net::TcpListener};
use sqlx::PgPool;
use crate::{
    configuration::Configuration, configuration::CONFIGURATION_FILE,
    repositories::currency_repository::CurrencyRepository,
    repositories::custodian_repository::CustodianRepository};
//use utils::routing;
mod configuration;
mod endpoints;
mod entities;
mod repositories;

#[derive(Clone)]
pub struct AppState {
    pub config: configuration::Configuration,
    pub currency_repository: CurrencyRepository,
    pub custodian_repository: CustodianRepository,
}


// The tokio::main macro is used to run the async main function
#[tokio::main]
async fn main() {

    // production read the configuration file from an environment variable,
    // for local debug it use a (local) configuration file if exists
    let config_file = match std::fs::exists(CONFIGURATION_FILE) {
        Ok(true) => { 
            println!("Using configuration file '{}'.", CONFIGURATION_FILE); 
            String::from(CONFIGURATION_FILE)
        },
        Ok(false) => { 
            println!("Configuration file '{}' not found, using CONFIGURATION_FILE environment variable.", CONFIGURATION_FILE); 
            std::env::var("CONFIGURATION_FILE")
                .expect("CONFIGURATION_FILE environment variable must be set (.env file can be used to set it).")
        },
        Err(e) => panic!("Failed to check for local configuration file '{}': {}", CONFIGURATION_FILE, e),
    };

    //eprintln!("Current dir: {:?}", std::env::current_dir());

    println!("Load configuration from '{}'", config_file);

    let config = Configuration::load_from_json_file(&config_file)
        .expect("Failed to create Configuration");

    println!("Configuration loaded for environment '{}'", config.environment);

    println!("Conenct to database...");
    let db_pool = PgPool::connect(&config.database_connection_string)
        .await
        .unwrap_or_else(|e| panic!(
            "Failed to create database connection pool. Connection string: '{}'. {}",
            config.database_connection_string, e
        ));
    println!("...done");

    // Run database migrations if enabled in configuration
    if config.run_database_migrations {
        println!("Running database migrations...");
        match sqlx::migrate!("./migrations").run(&db_pool).await {
            Ok(_) => println!("Database migrations applied successfully."),
            Err(e) => panic!("Failed to apply database migrations: {}", e),
        }
    } else {
        println!("Database migrations are disabled in configuration.");
    }

    let app_state = AppState {
        config: config.clone(),
        currency_repository: CurrencyRepository::new(db_pool.clone()),
        custodian_repository: CustodianRepository::new(db_pool),
    };

    let app = Router::new()
        .route("/", get(endpoints::common::home))
        .route("/login", get(endpoints::auth::login))
        .route("/currency", post(endpoints::currency::create))
        .route("/currency", put(endpoints::currency::update))
        .route("/currency/{id}", get(endpoints::currency::single))
        .route("/currency", get(endpoints::currency::list))        
        .route("/custodian", post(endpoints::custodian::create))
        .route("/custodian", put(endpoints::custodian::update))
        .route("/custodian", get(endpoints::custodian::list))        
        .with_state(app_state)  // injection
        .layer(
            // CORS
            CorsLayer::new()
                .allow_origin([
                    HeaderValue::from_static("http://localhost:5173"), 
                    HeaderValue::from_static("https://mercurius.work")
                ])
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST, 
                    http::Method::PUT, 
                    http::Method::PATCH, 
                    http::Method::DELETE
                ])
                .allow_headers(Any)
        );

    // read the port from environment variable or use a default
    //let port = std::env::var("PORT")
    //    .unwrap_or_else(|_| "3000".to_string())
    //    .parse::<u16>()
    //    .expect("Failed to parse PORT environment variable as a number");
    
    // Bind on server (Azure or private linux Docker container) requires 0.0.0.0
    // it will bind 127.0.0.1 and localhost locally 
    let address = format!("0.0.0.0:{}", config.server_port);

    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind to address");

    println!("Server running at http://{}", listener.local_addr().unwrap().to_string().replace("0.0.0.0", "127.0.0.1"));

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// Axum handles the conversion of a simple string to the HTTP response
//async fn home() -> &'static str {
//    "Hello, Axum API (learning.Rust)!"
//}

