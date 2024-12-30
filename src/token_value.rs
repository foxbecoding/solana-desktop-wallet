use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

// Struct for the token price response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenData {
    pub id: String,
    pub token_type: String,
    pub price: String,
}

// Struct for the full response
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub data: HashMap<String, TokenData>,
    pub time_taken: f64,
}

// The main struct
pub struct TokenValue {
    pub ids: Vec<String>,
    pub prices: HashMap<String, TokenData>,
}
