use autoi18n_config::CliConfig;
use clap::Parser;
use commands::{Cli, CliCommand};
use config::CONFIG_PATH;
use error::CliError;

mod commands;
mod completions;
mod config;
mod error;
mod init;
mod pull;

#[inline]
fn _main() -> Result<(), CliError> {
    match Cli::parse().command {
        CliCommand::Init(arguments) => init::run(&arguments).map_err(CliError::from),
        CliCommand::Pull => {
            let config = CliConfig::load(CONFIG_PATH)?;

            pull::run(&config)
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
