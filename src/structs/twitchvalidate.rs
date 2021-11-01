use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TwitchValidate {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub expires_in: u64,
}
