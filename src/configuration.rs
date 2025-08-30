use std::fs;
use serde::Deserialize;
//use serde_json::Result; // alias for `pub type Result<T> = std::result::Result<T, Error>;`

#[derive(Deserialize, Clone)]
pub struct Configuration {
    environment: String,
    server_port: u16,
    database_connection_string: String,
    admin_email: String
}

impl Configuration {
    pub fn load_from_json_file(file:&str) -> Result<Configuration, String> {

        let content = fs::read_to_string(file)
            //.expect(&format!("Failed to read configuration file '{}'", file));  <-- it executes format ALWAYS
            //.unwrap_or_else(|e| panic!("Failed to read configuration file '{}': {}", file, e)); immediate panic
            .map_err(|e| format!("Failed to read configuration file '{}': {}", file, e))?;

        let config:Configuration = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize configuration file '{}': {}", file, e))?;

        Ok(config)
    }
}