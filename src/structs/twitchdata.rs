use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchData<T> {
    pub data: Vec<T>,
}
