#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    NotFound,
}

impl std::error::Error for ConfigError {}

impl core::fmt::Display for ConfigError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => e.fmt(f),
            Self::ParseError(e) => e.fmt(f),
            Self::NotFound => write!(f, "Config not found"),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::IoError(value),
        }
    }
}

impl From<serde_json::Error> for ConfigError {
    #[inline]
    fn from(value: serde_json::Error) -> Self {
        Self::ParseError(value)
    }
}
