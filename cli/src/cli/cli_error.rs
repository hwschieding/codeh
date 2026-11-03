use std::fmt;

#[derive(Debug)]
pub enum CliError {
    _BadParse(&'static str),
    NoValue(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::_BadParse(msg) => write!(f, "{msg}"),
            Self::NoValue(msg) => write!(f, "{msg}"),
        }
    }
}
impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
