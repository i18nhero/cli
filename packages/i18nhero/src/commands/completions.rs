use clap::Args;

#[derive(Args, Debug)]
pub struct CompletionsCommandArguments {
    pub shell: clap_complete::Shell,
}
