use clap::Args;

#[derive(Args, Debug)]
pub struct PullCommandArguments {
    #[arg(long, hide = true)]
    pub web_api_host: Option<String>,
}
