use autoi18n_config::CliConfig;

mod error;

#[inline]
fn _main() -> Result<(), error::CliError> {
    let _config = CliConfig::load("autoi18n.json")?;

    Ok(())
}

fn main() {
    if let Err(error) = _main() {
        eprintln!("{error}");
    }
}
