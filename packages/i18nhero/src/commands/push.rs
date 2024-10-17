use clap::Args;

#[derive(Args, Debug)]
pub struct PushCommandArguments {
    /// Use for authentication instead of global auth config.
    #[arg(long)]
    pub api_key: Option<String>,

    #[arg(long, hide = true)]
    pub web_api_host: Option<String>,
}
