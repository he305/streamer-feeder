use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlatformData {
    pub name: String,
    pub url: String
}