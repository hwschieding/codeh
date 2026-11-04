use std::fmt;

#[derive(Debug)]
pub(crate) enum LlmError {
    BadConnection(std::io::Error),
    Unknown(String),
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadConnection(err) => write!(f, "{err}"),
            Self::Unknown(msg) => write!(f, "Unknown error: {msg}"),
        }
    }
}

impl std::error::Error for LlmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadConnection(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for LlmError {
    fn from(value: std::io::Error) -> Self {
        Self::BadConnection(value)
    }
}
