use std::fmt::{self};

#[derive(PartialEq)]
pub enum TwitchError {
    TokenInvalid,
    AuthQueryInvalid,
    ServerUnavailable,
    InternalError,
}

impl fmt::Debug for TwitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenInvalid => write!(f, "TokenInvalid"),
            Self::AuthQueryInvalid => write!(f, "AuthQueryInvalid"),
            Self::ServerUnavailable => write!(f, "ServerUnavailable"),
            Self::InternalError => write!(f, "InternalError"),
        }
    }
}

impl fmt::Display for TwitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
