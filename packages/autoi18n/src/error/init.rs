#[derive(Debug)]
pub enum InitError {
    Io(std::io::Error),
    Serialization(serde_json::Error),
    ConfigAlreadyExists,
}

impl std::error::Error for InitError {}

impl core::fmt::Display for InitError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Serialization(e) => e.fmt(f),
            Self::ConfigAlreadyExists => write!(f, "A configuration file already exists"),
        }
    }
}

impl From<std::io::Error> for InitError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for InitError {
    #[inline]
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}
