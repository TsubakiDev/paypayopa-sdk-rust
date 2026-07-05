use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingAuth,
    Validation(String),
    Http(reqwest::Error),
    Json(serde_json::Error),
    Jwt(jsonwebtoken::errors::Error),
    Base64(base64::DecodeError),
}

impl Error {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAuth => write!(f, "missing API key and secret"),
            Self::Validation(message) => write!(f, "{message}"),
            Self::Http(error) => write!(f, "{error}"),
            Self::Json(error) => write!(f, "{error}"),
            Self::Jwt(error) => write!(f, "{error}"),
            Self::Base64(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(error)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Self::Base64(error)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ServerError {
    message: Option<String>,
}

impl ServerError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }

    pub fn without_message() -> Self {
        Self { message: None }
    }
}

impl Default for ServerError {
    fn default() -> Self {
        Self::without_message()
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{message}"),
            None => write!(f, "server error"),
        }
    }
}

impl std::error::Error for ServerError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignatureVerificationError {
    message: Option<String>,
}

impl SignatureVerificationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }

    pub fn without_message() -> Self {
        Self { message: None }
    }
}

impl Default for SignatureVerificationError {
    fn default() -> Self {
        Self::without_message()
    }
}

impl fmt::Display for SignatureVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{message}"),
            None => write!(f, "signature verification error"),
        }
    }
}

impl std::error::Error for SignatureVerificationError {}
