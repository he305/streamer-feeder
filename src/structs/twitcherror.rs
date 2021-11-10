use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchErrorMessage {
    pub status: i32,
    pub message: String,
}
