use serde::{Deserialize};

#[derive(Deserialize)]
pub struct TokenInfo {
    pub token: String,
    pub token_type: String
}