use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

// Struct for the token price response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenData {
    pub id: String,
    pub price: String,
}

impl TokenData {
    /// Get the price formatted to two decimal places
    pub fn formatted_price(&self) -> String {
        if let Ok(price) = self.price.parse::<f64>() {
            format!("{:.2}", price)
        } else {
            self.price.clone() // Fallback to the original string if parsing fails
        }
    }
}

// Struct for the full response
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub data: HashMap<String, TokenData>,
    #[serde(rename = "timeTaken")]
    pub time_taken: Option<f64>,
}

// The main struct
pub struct TokenValue {
    pub ids: Vec<String>,
    pub prices: HashMap<String, TokenData>,
}

impl TokenValue {
    /// Create a new TokenValue and fetch prices for the given keys
    pub async fn new(ids: &[&str]) -> Result<Self, Box<dyn Error>> {
        // Construct the URL with all keys
        let url = format!("https://api.jup.ag/price/v2?ids={}", ids.join(","));

        // Make the HTTP request
        let response: TokenResponse = reqwest::get(&url).await?.json().await?;

        Ok(TokenValue {
            ids: ids.iter().map(|&key| key.to_string()).collect(),
            prices: response.data,
        })
    }

    /// Get price information for a specific key
    pub fn get_price(&self, id: &str) -> Option<&TokenData> {
        self.prices.get(id)
    }
}
