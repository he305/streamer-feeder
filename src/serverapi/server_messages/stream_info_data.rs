use serde::{Serialize, Deserialize};

use super::ChannelData;

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamInfoData {
    pub channel: ChannelData,
    #[serde(rename = "gameName")]
    pub game_name: String,
    pub title: String,
    #[serde(rename = "viewerCount")]
    pub viewer_count: i32,
    #[serde(rename = "isLive")]
    pub is_live: bool
}