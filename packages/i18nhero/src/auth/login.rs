use dialoguer::theme::ColorfulTheme;

use crate::{
    error::CliError,
    terminal::{hyperlink, print_logged_in, print_not_authenticated},
};

use super::AuthConfig;

#[inline]
fn prompt_api_key() -> Result<String, dialoguer::Error> {
    dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "What is your api key? ({})",
            hyperlink(
                "https://i18nhero.com/settings/api",
                "https://i18nhero.com/settings/api"
            )
        ))
        .allow_empty(false)
        .validate_with(|v: &String| -> Result<(), &str> {
            if uuid::Uuid::parse_str(v.trim()).is_ok() {
                Ok(())
            } else {
                Err("Invalid api key")
            }
        })
        .interact()
}

#[inline]
pub fn run() -> Result<AuthConfig, CliError> {
    let api_key = prompt_api_key()?;

    let auth = AuthConfig::new(api_key.trim().to_owned());

    auth.save()?;

    print_logged_in();

    Ok(auth)
}

#[inline]
pub fn prompt_should_login() -> Result<bool, dialoguer::Error> {
    print_not_authenticated();

    dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to login now?")
        .default(true)
        .interact()
}
