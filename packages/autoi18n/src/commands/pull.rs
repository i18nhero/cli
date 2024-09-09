use clap::Args;

#[derive(Args, Debug)]
pub struct PullCommandArguments {
    #[arg(long, hide = true)]
    pub api_host: Option<String>,
}
