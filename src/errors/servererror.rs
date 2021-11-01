use std::fmt;

#[derive(PartialEq)]
pub enum ServerError {
    ServerAuthError(AuthError),
    ServerUnavailable,
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
            Self::ServerUnavailable => write!(f, "ServerUnavailable"),
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
