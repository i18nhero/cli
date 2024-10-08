use clap::Parser;
use commands::{Cli, CliCommand};
use config::{CliConfig, CONFIG_PATH};
use error::CliError;

mod auth;
mod commands;
mod completions;
mod config;
mod error;
mod init;
mod pull;
mod push;
mod terminal;

pub const DEFAULT_API_HOST: &str = "https://cli.api.i18nhero.com";

pub const DEFAULT_WEB_API_HOST: &str = "https://web.api.i18nhero.com";

#[inline]
async fn _main() -> Result<(), CliError> {
    match Cli::parse().command {
        CliCommand::Login => auth::login::run().map(|_api_key| ()),
        CliCommand::Logout => auth::logout::run(),
        CliCommand::Init(arguments) => init::run(&arguments).await,
        CliCommand::Pull(arguments) => {
            let config = CliConfig::load(CONFIG_PATH)?;

            pull::run(&arguments, &config).await
        }
        CliCommand::Push(arguments) => {
            let config = CliConfig::load(CONFIG_PATH)?;

            push::run(&arguments, &config).await
        }
        CliCommand::Completions(arguments) => {
            completions::run(&arguments);

            Ok(())
        }
    }
}

#[tokio::main]
#[expect(clippy::needless_return)]
async fn main() {
    if let Err(error) = _main().await {
        terminal::print_error(&error);

        std::process::exit(1);
    }
}
