use clap::CommandFactory;

use crate::commands::{
    completions::{CompletionsCommandArguments, TerminalShell},
    Cli,
};

#[inline]
pub fn run(args: &CompletionsCommandArguments) {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    match args.shell {
        TerminalShell::Bash => clap_complete::generate(
            clap_complete::Shell::Bash,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        TerminalShell::Elvish => clap_complete::generate(
            clap_complete::Shell::Elvish,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        TerminalShell::PowerShell => clap_complete::generate(
            clap_complete::Shell::PowerShell,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        TerminalShell::Fish => clap_complete::generate(
            clap_complete::Shell::Fish,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        TerminalShell::Zsh => clap_complete::generate(
            clap_complete::Shell::Zsh,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),

        TerminalShell::Nushell => clap_complete::generate(
            clap_complete_nushell::Nushell,
            &mut cmd,
            cmd_name,
            &mut std::io::stdout(),
        ),
    }
}
