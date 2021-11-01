use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TwitchAuth {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}
