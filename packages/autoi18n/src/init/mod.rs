use autoi18n_config::CliConfig;

use crate::{commands::init::InitCommandArguments, config::CONFIG_PATH, error::init::InitError};

#[inline]
pub fn run(arguments: &InitCommandArguments) -> Result<(), InitError> {
    if !arguments.overwrite && std::fs::exists(CONFIG_PATH)? {
        return Err(InitError::ConfigAlreadyExists);
    }

    let config = CliConfig::default();

    let mut json = serde_json::to_string_pretty(&config)?;

    json.push('\n');

    std::fs::write(CONFIG_PATH, json)?;

    println!("Configuration file has been created");

    Ok(())
}
