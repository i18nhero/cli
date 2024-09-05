use error::ConfigError;

pub mod error;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CliConfig {
    #[serde(rename = "$schema", default = "CliConfig::default_schema_location")]
    pub schema: String,
}

impl CliConfig {
    #[inline]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;

        let parsed = serde_json::from_str::<Self>(&content)?;

        Ok(parsed)
    }

    #[inline]
    fn default_schema_location() -> String {
        let package_version = env!("CARGO_PKG_VERSION");

        format!(
        "https://raw.githubusercontent.com/autoi18n/cli/main/schemas/v{package_version}/autoi18n.schema.json"
    )
    }
}
