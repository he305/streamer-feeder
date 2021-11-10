use crate::serverapi::server_messages::{ChannelData};

pub struct StreamerData {
    pub channel: ChannelData,
    pub game_name: String,
    pub title: String,
    pub viewer_count: i32,
    pub is_live: bool,
    pub was_live: bool
}