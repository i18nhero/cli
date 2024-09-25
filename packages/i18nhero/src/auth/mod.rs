use crate::error::CliError;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthConfig {
    pub api_key: String,
}

const AUTH_CONFIG_FILE_NAME: &str = ".auth.json";

impl AuthConfig {
    #[inline]
    fn get_config_dir() -> std::path::PathBuf {
        // TODO: handle None?
        dirs::config_dir()
            .or_else(dirs::home_dir)
            .unwrap()
            .join(".i18nhero")
    }

    #[inline]
    pub fn load() -> Result<Self, CliError> {
        let dir = AuthConfig::get_config_dir();

        let _ = std::fs::create_dir_all(&dir);

        let contents = std::fs::read_to_string(dir.join(AUTH_CONFIG_FILE_NAME))
            .map_err(CliError::AuthConfigLoad)?;

        serde_json::from_str::<AuthConfig>(&contents).map_err(CliError::AuthConfigDeserialize)
    }

    #[inline]
    fn save(&self) -> Result<(), CliError> {
        let contents = serde_json::to_string_pretty(self).map_err(CliError::AuthConfigSerialize)?;

        let dir = Self::get_config_dir();

        let _ = std::fs::create_dir_all(&dir);

        std::fs::write(dir.join(AUTH_CONFIG_FILE_NAME), contents)
            .map_err(CliError::AuthConfigSave)?;

        Ok(())
    }
}
