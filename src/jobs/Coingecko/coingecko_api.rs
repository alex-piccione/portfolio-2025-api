use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

pub struct CoingeckoApi {
    config: Configuration,
    client: Client,
}

const api_url:str = "https://api.coingecko.com/api/v3";

impl CoingeckoApi {
    // Constructor
    pub fn new(config: Configuration) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    // Ping endpoint (GET /ping)
    pub fn ping(&self) -> bool {
        let url = format!("{}/ping", api_url);

        // Set up headers with API key "x-cg-demo-api-key"
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-cg-demo-api-key",
            HeaderValue::from_str(&self.config.secrets.coingecko_api_key),
        );

        // Make the GET request
        let resp = self.client
            .get(url)
            .headers(headers)
            .send();

        match resp {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}
