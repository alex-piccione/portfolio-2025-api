use reqwest::{Client, header::{HeaderMap, HeaderValue}};

use crate::configuration::Configuration;

#[derive(Clone)]
pub struct CoingeckoApi {
    config: Configuration,
    client: Client,
}

const API_URL: &'static str = "https://api.coingecko.com/api/v3";

impl CoingeckoApi {

    pub fn new(config: &Configuration) -> Self {
        Self {
            config: config.clone(),
            client: Client::new(),
        }
    }

    // Ping endpoint (GET /ping)
    pub async fn ping(&self) -> bool {
        let url = format!("{}/ping", API_URL);

        // Set up headers with API key "x-cg-demo-api-key"
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-cg-demo-api-key",
            HeaderValue::from_str(&self.config.secrets.coingecko_api_key).unwrap(),
        );

        // Make the GET request
        let resp = self.client
            .get(url)
            .headers(headers)
            .send()
            .await;

        match resp {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}
