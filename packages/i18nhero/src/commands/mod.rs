use clap::{Parser, Subcommand};
use completions::CompletionsCommandArguments;
use init::InitCommandArguments;
use pull::PullCommandArguments;
use push::PushCommandArguments;

pub mod completions;
pub mod init;
pub mod pull;
pub mod push;

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{about-with-newline}{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true, help_template = HELP_TEMPLATE)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    /// Login to i18nhero
    Login,

    /// Logout from i18nhero
    Logout,

    /// Pull locales
    Pull(PullCommandArguments),

    /// Push locale keys to i18nhero
    Push(PushCommandArguments),

    /// Create new i18nhero config
    Init(InitCommandArguments),

    /// Shell completions
    Completions(CompletionsCommandArguments),
}
