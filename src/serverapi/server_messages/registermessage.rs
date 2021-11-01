use serde::{Deserialize};

use super::{TokenInfo};

#[derive(Deserialize)]
pub struct RegisterMessage {
    #[serde(rename = "tokenInfo")]
    pub token_info: TokenInfo,
    #[serde(rename = "appName")]
    pub app_name: String,
    pub password: String
}
