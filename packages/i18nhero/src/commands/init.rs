use clap::Args;

#[derive(Args, Debug)]
pub struct InitCommandArguments {
    #[arg(long, default_value_t = false)]
    pub overwrite: bool,

    #[arg(long, hide = true)]
    pub cli_api_host: Option<String>,
}
