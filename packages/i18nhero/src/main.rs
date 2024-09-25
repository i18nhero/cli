use clap::Parser;
use commands::{Cli, CliCommand};
use config::CONFIG_PATH;
use error::CliError;
use i18nhero_config::CliConfig;

mod auth;
mod commands;
mod completions;
mod config;
mod error;
mod generators;
mod init;
mod pull;
mod push;
mod terminal;

pub const DEFAULT_API_HOST: &str = "https://api.i18nhero.com";

pub const DEFAULT_WEB_API_HOST: &str = "https://web.api.i18nhero.com";

#[inline]
async fn _main() -> Result<(), CliError> {
    match Cli::parse().command {
        CliCommand::Init(arguments) => init::run(&arguments).await.map_err(CliError::from),
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

#[tokio::main]
async fn main() {
    if let Err(error) = _main().await {
        terminal::print_error(&error);

        std::process::exit(1)
    }
}
