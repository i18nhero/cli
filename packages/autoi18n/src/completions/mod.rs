use clap::CommandFactory;

use crate::commands::{completions::CompletionsCommandArguments, Cli};

#[inline]
pub fn run(args: &CompletionsCommandArguments) {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    clap_complete::generate(args.shell, &mut cmd, cmd_name, &mut std::io::stdout());
}
