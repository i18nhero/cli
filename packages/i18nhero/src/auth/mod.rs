use login::prompt_should_login;

use crate::error::CliError;

pub mod login;
pub mod logout;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthConfig {
    pub api_key: String,
}

impl AuthConfig {
    #[inline]
    const fn new(api_key: String) -> Self {
        Self { api_key }
    }

    #[inline]
    fn get_auth_config_path() -> std::path::PathBuf {
        // TODO: handle None?
        let dir = dirs::config_dir()
            .or_else(dirs::home_dir)
            .unwrap()
            .join(".i18nhero");

        let _ = std::fs::create_dir_all(&dir);

        dir.join(".auth.json")
    }

    #[inline]
    pub fn load() -> Result<Self, CliError> {
        let path = Self::get_auth_config_path();

        if !std::fs::exists(&path).map_err(CliError::Io)? {
            if prompt_should_login() {
                return login::run();
            }

            std::process::exit(0);
        }

        let contents = std::fs::read_to_string(path).map_err(CliError::AuthConfigLoad)?;

        serde_json::from_str(&contents).map_err(CliError::AuthConfigDeserialize)
    }

    #[inline]
    fn save(&self) -> Result<(), CliError> {
        let contents = serde_json::to_string_pretty(self).map_err(CliError::AuthConfigSerialize)?;

        let path = Self::get_auth_config_path();

        std::fs::write(path, contents).map_err(CliError::AuthConfigSave)?;

        Ok(())
    }

    #[inline]
    fn remove() -> std::io::Result<()> {
        let path = Self::get_auth_config_path();

        if std::fs::exists(&path)? {
            std::fs::remove_file(&path)
        } else {
            Ok(())
        }
    }
}
