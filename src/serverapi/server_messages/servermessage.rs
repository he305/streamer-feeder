use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ServerMessage<T> {
    pub error: String,
    pub data: Vec<T>,
}
