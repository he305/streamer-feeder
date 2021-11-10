use serde::{Serialize, Deserialize};

use super::{PersonData, PlatformData};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChannelData {
    pub person: PersonData,
    pub platform: PlatformData,
    #[serde(rename = "broadcastId")]
    pub broadcast_id: Option<i32>,
    #[serde(rename = "broadcastName")]
    pub broadcast_name: String
}