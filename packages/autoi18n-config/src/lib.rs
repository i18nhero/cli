use error::ConfigError;

pub mod error;

#[derive(serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
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

#[cfg(test)]
mod test_config {

    use crate::CliConfig;

    #[test]
    fn config_should_be_serializable() {
        let config = CliConfig::default();

        let json = serde_json::to_string_pretty(&config).expect("it to be serializable");

        let file = tempfile::Builder::new()
            .suffix(".json")
            .tempfile()
            .expect("it to create file");

        std::fs::write(file.path(), json).expect("it to write to file");

        let loaded = CliConfig::load(file.path()).expect("it to be parsed");

        assert_eq!(config, loaded);
    }

    #[test]
    #[cfg(feature = "json-schema")]
    fn json_schema_should_be_serializable() {
        serde_json::to_string_pretty(&schemars::schema_for!(CliConfig))
            .expect("it to be serializable");
    }
}
