use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: u64,
    #[serde(rename = "appName")]
    pub app_name: String,
    pub password: String,
}
