use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlatformData {
    pub name: String,
    pub url: String
}