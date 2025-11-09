use tokio::{net::TcpListener};
use sqlx::PgPool;
use crate::{
    configuration::{Configuration}, 
    utils::{cors::RouterExtensions as _, dependency_injection}};

mod configuration;
mod constants;
mod utils;
mod endpoints;
mod entities;
mod services;
mod repositories;
mod jobs;

// The tokio::main macro is used to run the async main function
#[tokio::main]
async fn main() {

    let config_file = std::env::var("CONFIGURATION_FILE")
        .expect("CONFIGURATION_FILE environment variable must be set (.env file can be used to set it).");

    //eprintln!("Current dir: {:?}", std::env::current_dir());
    println!("CONFIGURATION_FILE: '{}'", config_file);

    let config = Configuration::load_from_json_file(&config_file)
        .expect("Failed to create Configuration");

    println!("Configuration loaded for environment '{}'", config.environment);

    println!("Connect to database...");
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

    let app_state = dependency_injection::inject_services(&config, db_pool).await;
        
    println!("setup jobs...");
    jobs::job_manager::schedule_jobs(&config, app_state.clone()).await;
    println!("jobs setup completed");

    let app = utils::routing::set_routes(app_state.clone())
        .with_state(app_state)
        .set_cors(&config.app_domain);
   
    // Bind on server (Azure or Docker container) requires 0.0.0.0 
    // LOcally it will bind 127.0.0.1 and localhost.
    let address = format!("0.0.0.0:{}", &config.server_port);

    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind to address");

    println!("Server running at http://{}", listener.local_addr().unwrap().to_string().replace("0.0.0.0", "127.0.0.1"));

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
