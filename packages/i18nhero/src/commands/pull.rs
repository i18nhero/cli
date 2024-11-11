use clap::Args;

#[derive(Args, Debug)]
pub struct PullCommandArguments {
    /// Allow overwriting files that are not staged in VCS.
    ///
    /// Currently only applicable for projects that use git.
    #[arg(long, default_value_t = false)]
    pub allow_dirty: bool,

    /// Use for authentication instead of global auth config.
    #[arg(long)]
    pub api_key: Option<String>,

    #[arg(long, hide = true)]
    pub api_host: Option<String>,
}
