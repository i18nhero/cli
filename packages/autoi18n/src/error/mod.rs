use init::InitError;

pub mod init;

#[derive(Debug)]
pub enum CliError {
    Config(autoi18n_config::error::ConfigError),
    Init(InitError),
}

impl std::error::Error for CliError {}

impl core::fmt::Display for CliError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(e) => e.fmt(f),
            Self::Init(e) => e.fmt(f),
        }
    }
}

impl From<autoi18n_config::error::ConfigError> for CliError {
    #[inline]
    fn from(value: autoi18n_config::error::ConfigError) -> Self {
        Self::Config(value)
    }
}

impl From<InitError> for CliError {
    #[inline]
    fn from(value: InitError) -> Self {
        Self::Init(value)
    }
}
