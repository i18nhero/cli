use clap::Args;

#[derive(Args, Debug)]
pub struct PushCommandArguments {
    #[arg(long, hide = true)]
    pub api_host: Option<String>,
}
