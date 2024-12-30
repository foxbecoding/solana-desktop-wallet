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
