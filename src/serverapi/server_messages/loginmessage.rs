use serde::{Deserialize};

use super::TokenInfo;

#[derive(Deserialize)]
pub struct LoginMessage {
    pub token: TokenInfo,
}
