// import the necessary modules
use axum::{
    routing::get,
    routing::post,
    Router,
};

use tokio::{net::TcpListener};

use crate::configuration::Configuration;

mod endpoints;
mod entities;
mod configuration;
//use crate::configuration::Configuration;

#[derive(Clone)]
pub struct AppState {
    pub config: configuration::Configuration
}

// The tokio::main macro is used to run the async main function
#[tokio::main]
async fn main() {

    // use a local configuration file if exists (for local debug)
    let config_file = match std::fs::exists("configuration.json") {
        Ok(true) => { 
            println!("Using local configuration file 'configuration.json'"); 
            String::from("configuration.json")
        },
        Ok(false) => { 
            println!("No local configuration file 'configuration.json' found, using CONFIGURATION_FILE environment variable"); 
            std::env::var("CONFIGURATION_FILE")
                .expect("CONFIGURATION_FILE environment variable must be set")
        },
        Err(e) => panic!("Failed to check for local configuration file 'configuration.json': {}", e),
    };

    println!("Load configuration from '{}'", config_file);

    let config = Configuration::load_from_json_file(&config_file)
        .expect("Failed to create Configuration");

    println!("Configuration loaded for environment '{}'", config.environment);

    let app = Router::new()
        .route("/", get(endpoints::common::home))
        .route("/currency", get(endpoints::currency::list))
        .route("/currency/{id}", get(endpoints::currency::single))
        .route("/currency", post(endpoints::currency::create));

    // read the port from environment variable or use a default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Failed to parse PORT environment variable as a number");
    
    // Bind on server (Azure or private linux Docker container) requires 0.0.0.0
    // it will bind 127.0.0.1 and localhost locally 
    let address = format!("0.0.0.0:{}", port);

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