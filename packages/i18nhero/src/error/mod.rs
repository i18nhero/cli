#[derive(Debug)]
pub enum CliError {
    Config(i18nhero_config::error::ConfigError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
    SerdeYml(serde_yml::Error),
    Json5(json5::Error),
    Reqwest(reqwest::Error),
    ConfigAlreadyExists,
    MissingProjectId,
}

impl std::error::Error for CliError {}

impl core::fmt::Display for CliError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::SerdeJson(e) => e.fmt(f),
            Self::SerdeYml(e) => e.fmt(f),
            Self::Json5(e) => e.fmt(f),
            Self::Reqwest(e) => e.fmt(f),

            Self::ConfigAlreadyExists => write!(f, "A configuration file already exists"),
            Self::MissingProjectId => write!(f, "project_id must be set in config"),
        }
    }
}

impl From<i18nhero_config::error::ConfigError> for CliError {
    #[inline]
    fn from(value: i18nhero_config::error::ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<std::io::Error> for CliError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for CliError {
    #[inline]
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<reqwest::Error> for CliError {
    #[inline]
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_yml::Error> for CliError {
    #[inline]
    fn from(value: serde_yml::Error) -> Self {
        Self::SerdeYml(value)
    }
}

impl From<json5::Error> for CliError {
    #[inline]
    fn from(value: json5::Error) -> Self {
        Self::Json5(value)
    }
}
