use crate::{config::CliConfigOutputFormat, error::CliError};

pub fn parse_input(
    format: &CliConfigOutputFormat,
    contents: &str,
) -> Result<std::collections::HashMap<String, String>, CliError> {
    match format {
        CliConfigOutputFormat::Json => {
            serde_json::from_str(contents).map_err(CliError::DeserializeJson)
        }
        CliConfigOutputFormat::Json5 => {
            json5::from_str(contents).map_err(CliError::DeserializeJson5)
        }
        CliConfigOutputFormat::Yaml => {
            serde_yml::from_str(contents).map_err(CliError::DeserializeYml)
        }
    }
}

pub fn stringify(
    format: &CliConfigOutputFormat,
    translations: &std::collections::BTreeMap<String, String>,
) -> Result<String, CliError> {
    match format {
        CliConfigOutputFormat::Json => {
            serde_json::to_string_pretty(&translations).map_err(CliError::SerializeJson)
        }
        CliConfigOutputFormat::Json5 => {
            json5::to_string(&translations).map_err(CliError::SerializeJson5)
        }
        CliConfigOutputFormat::Yaml => {
            serde_yml::to_string(&translations).map_err(CliError::SerializeYml)
        }
    }
    .map(|output| {
        let mut trimmed = output.trim().to_string();
        trimmed.push('\n');
        trimmed
    })
}
