use autoi18n_config::CliConfig;

use crate::{commands::init::InitCommandArguments, config::CONFIG_PATH, error::CliError};

#[inline]
pub fn run(arguments: &InitCommandArguments) -> Result<(), CliError> {
    if !arguments.overwrite && std::fs::exists(CONFIG_PATH)? {
        return Err(CliError::ConfigAlreadyExists);
    }

    let config = CliConfig::default();

    let mut json = serde_json::to_string_pretty(&config)?;

    json.push('\n');

    std::fs::write(CONFIG_PATH, json)?;

    println!("Configuration file has been created");

    Ok(())
}
