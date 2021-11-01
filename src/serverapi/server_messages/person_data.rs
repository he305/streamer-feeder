use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonData {
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub priority: u16
}
