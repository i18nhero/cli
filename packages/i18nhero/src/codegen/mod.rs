#[allow(clippy::all)]
#[allow(dead_code)]
pub mod cli_api;
#[allow(clippy::all)]
#[allow(dead_code)]
pub mod web_api;

#[inline]
fn default_user_agent() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!("@i18nhero/cli/v{package_version} +http://i18nhero.com")
}

const DEFAULT_CLI_API_HOST: &str = "https://cli.api.i18nhero.com";

#[inline]
pub fn setup_web_api_configuration(
    host: Option<String>,
) -> web_api::apis::configuration::Configuration {
    web_api::apis::configuration::Configuration {
        base_path: host.unwrap_or_else(|| DEFAULT_WEB_API_HOST.to_owned()),
        user_agent: Some(default_user_agent()),
        ..Default::default()
    }
}

const DEFAULT_WEB_API_HOST: &str = "https://web.api.i18nhero.com";

#[inline]
pub fn setup_cli_api_configuration(
    host: Option<String>,
) -> cli_api::apis::configuration::Configuration {
    cli_api::apis::configuration::Configuration {
        base_path: host.unwrap_or_else(|| DEFAULT_CLI_API_HOST.to_owned()),
        user_agent: Some(default_user_agent()),
        ..Default::default()
    }
}
