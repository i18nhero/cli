use clap::Args;

#[derive(Args, Debug)]
pub struct PullCommandArguments {
    #[arg(long, default_value_t = false)]
    pub allow_dirty: bool,

    #[arg(long, hide = true)]
    pub web_api_host: Option<String>,
}
