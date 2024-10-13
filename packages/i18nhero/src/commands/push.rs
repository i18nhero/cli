use clap::Args;

#[derive(Args, Debug)]
pub struct PushCommandArguments {
    #[arg(long, hide = true)]
    pub web_api_host: Option<String>,
}
