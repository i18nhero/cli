use clap::Parser;
use i18nhero::{
    auth,
    commands::{Cli, CliCommand},
    completions,
    config::{CliConfig, CONFIG_PATH},
    error::CliError,
    init, pull, push, terminal,
};

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
async fn main() {
    if let Err(error) = _main().await {
        terminal::print_error(&error);

        std::process::exit(1);
    }
}
