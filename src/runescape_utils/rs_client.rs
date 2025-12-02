use reqwest::{Client, Url};
use serde::{Deserialize, de::Error};
use std::collections::HashMap;
use thiserror::Error;

const BASE_URL: &str = "https://api.weirdgloop.org";
const USER_AGENT: &str = concat!("rust-osrs-wiki-api-wrapper/", env!("CARGO_PKG_VERSION"));

pub struct RSClient {
    client: Client,
    base_url: Url,
    item_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RSItemPrice {
    pub item: String,
    pub price: u64,
    pub volume: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TimeStampValue {
    String(String),
    Number(u64),
}

#[derive(Debug, Deserialize, Clone)]
pub struct RSItemPriceHistory {
    pub item: String,
    pub history: Vec<RSPrice>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RSPrice {
    pub timestamp: TimeStampValue,
    pub price: u64,
    pub volume: u64,
}
pub type RSPriceMapResponse = HashMap<String, RSPrice>;
pub type RSPriceHistoryMapResponse = HashMap<String, Vec<RSPrice>>;

#[derive(Error, Debug)]
pub enum RSError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
}

impl Default for RSClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RSClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        RSClient {
            client,
            base_url: Url::parse(BASE_URL).unwrap(),
            item_name: String::new(),
        }
    }

    pub fn item_name(mut self, item_name: String) -> Self {
        self.item_name = item_name;
        self
    }

    pub async fn get_price(&self) -> Result<RSItemPrice, RSError> {
        let encoded_name = urlencoding::encode(&self.item_name);
        let path = format!(
            "/exchange/history/osrs/latest?name={}&lang=en",
            encoded_name
        );

        let url = self.base_url.join(&path).map_err(RSError::UrlParse)?;
        let response = self.client.get(url).send().await.map_err(RSError::Http)?;

        let body_text = response.text().await.map_err(RSError::Http)?;

        // The API returns a HashMap of item names to price data
        let price_map: RSPriceMapResponse =
            serde_json::from_str(&body_text).map_err(RSError::JsonParse)?;

        // Extract the first (and typically only) entry from the map
        let (item_name, price_data) = price_map
            .into_iter()
            .next()
            .ok_or_else(|| RSError::JsonParse(serde_json::Error::custom("empty response")))?;

        // Convert RSPrice to RSItemPrice by adding the item name
        let price_response = RSItemPrice {
            item: item_name,
            price: price_data.price,
            volume: price_data.volume,
        };

        Ok(price_response)
    }

    pub async fn get_price_history(&self) -> Result<RSItemPriceHistory, RSError> {
        let encoded_name = urlencoding::encode(&self.item_name);
        let path = format!(
            "/exchange/history/osrs/last90d?name={}&lang=en",
            encoded_name
        );

        let url = self.base_url.join(&path).map_err(RSError::UrlParse)?;
        let response = self.client.get(url).send().await.map_err(RSError::Http)?;

        let body_text = response.text().await.map_err(RSError::Http)?;

        // The API returns a HashMap of item names to arrays of price history data
        let price_history_map: RSPriceHistoryMapResponse = serde_json::from_str(&body_text)
            .map_err(|e| {
                eprintln!("JSON parse error at position: {}", e);
                eprintln!("Response body: {}", body_text);
                RSError::JsonParse(e)
            })?;

        // Extract the first (and typically only) entry from the map
        let (item_name, price_history) = price_history_map
            .into_iter()
            .next()
            .ok_or_else(|| RSError::JsonParse(serde_json::Error::custom("empty response")))?;

        let item_price_history = RSItemPriceHistory {
            item: item_name,
            history: price_history,
        };

        Ok(item_price_history)
    }
}
