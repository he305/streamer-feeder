use std::fmt::{self};

pub enum TwitchError {
    TokenInvalid,
    AuthQueryInvalid,
    ServerUnavailable(reqwest::Error),
    ResponseInvalid(serde_json::Error),
    InternalError,
}

impl fmt::Debug for TwitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenInvalid => write!(f, "TokenInvalid"),
            Self::AuthQueryInvalid => write!(f, "AuthQueryInvalid"),
            Self::ServerUnavailable(arg) => write!(f, "ServerUnavailable, error: {}", arg),
            Self::InternalError => write!(f, "InternalError"),
            Self::ResponseInvalid(arg) => write!(f, "Response invelid, error {}", arg)
        }
    }
}

impl fmt::Display for TwitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
