use crate::{error::CliError, terminal::print_logged_out};

use super::AuthConfig;

#[inline]
pub fn run() -> Result<(), CliError> {
    AuthConfig::remove().map_err(CliError::Io)?;

    print_logged_out();

    Ok(())
}
