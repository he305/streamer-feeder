use std::fmt;

pub enum ServerError {
    ServerAuthError(AuthError),
    ServerUnavailable(reqwest::Error),
    ResponseInvalid(serde_json::Error),
    RequestInvalid(String)
}

#[derive(PartialEq)]
pub enum AuthError {
    NotRegistered,
    UserNameAlreadyExists,
}

impl fmt::Debug for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRegistered => write!(f, "NotRegistered"),
            Self::UserNameAlreadyExists => write!(f, "UserNameAlreadyExists"),
        }
    }
}

impl fmt::Debug for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServerAuthError(arg0) => f.debug_tuple("ServerAuthError").field(arg0).finish(),
            Self::ServerUnavailable(arg) => write!(f, "Server Unavailable, error {}", arg),
            Self::ResponseInvalid(arg) => write!(f, "Invalid response, error {}", arg),
            Self::RequestInvalid(arg) => write!(f, "Request invalid, error {}", arg)
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
