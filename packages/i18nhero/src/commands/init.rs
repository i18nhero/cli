use clap::Args;

#[derive(Args, Debug)]
pub struct InitCommandArguments {
    #[arg(long, default_value_t = false)]
    pub overwrite: bool,
}
