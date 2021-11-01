use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TwitchStreamData {
    #[serde(rename = "game_id")]
    pub game_id: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    pub id: String,
    #[serde(rename = "is_mature")]
    pub is_mature: bool,
    pub language: String,
    #[serde(rename = "started_at")]
    pub started_at: String,
    #[serde(rename = "tag_ids")]
    pub tag_ids: Vec<String>,
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_login")]
    pub user_login: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "viewer_count")]
    pub viewer_count: i64,
}
