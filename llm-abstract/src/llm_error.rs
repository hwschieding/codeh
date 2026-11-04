use std::fmt;

#[derive(Debug)]
pub enum LlmError {
    BadRequest(reqwest::Error),
    Unknown(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(err) => write!(f, "{err}"),
            Self::Unknown(msg) => write!(f, "Unknown error: {msg}"),
        }
    }
}

impl std::error::Error for LlmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadRequest(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for LlmError {
    fn from(value: reqwest::Error) -> Self {
        Self::BadRequest(value)
    }
}
