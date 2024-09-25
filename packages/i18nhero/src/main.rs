use clap::Parser;
use commands::{Cli, CliCommand};
use config::CONFIG_PATH;
use error::CliError;
use i18nhero_config::CliConfig;

mod commands;
mod completions;
mod config;
mod error;
pub mod generators;
mod init;
mod pull;
mod push;

pub const DEFAULT_API_HOST: &str = "https://web.api.i18nhero.com";

#[inline]
fn _main() -> Result<(), CliError> {
    match Cli::parse().command {
        CliCommand::Init(arguments) => init::run(&arguments).map_err(CliError::from),
        CliCommand::Pull(arguments) => {
            let config = CliConfig::load(CONFIG_PATH)?;

            pull::run(&arguments, &config)
        }
        CliCommand::Push(arguments) => {
            let config = CliConfig::load(CONFIG_PATH)?;

            push::run(&arguments, &config)
        }
        CliCommand::Completions(arguments) => {
            completions::run(&arguments);

            Ok(())
        }
    }
}

fn main() {
    if let Err(error) = _main() {
        eprintln!("{error}");

        std::process::exit(1)
    }
}
