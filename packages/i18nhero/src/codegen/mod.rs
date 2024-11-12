#[allow(clippy::all)]
#[allow(dead_code)]
pub mod public_api;

#[inline]
fn default_user_agent() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!("@i18nhero/cli/v{package_version} +http://i18nhero.com")
}

const DEFAULT_API_HOST: &str = "https://api.i18nhero.com";

#[inline]
pub fn setup_api_configuration(
    host: Option<String>,
) -> public_api::apis::configuration::Configuration {
    public_api::apis::configuration::Configuration {
        base_path: host.unwrap_or_else(|| DEFAULT_API_HOST.to_owned()),
        user_agent: Some(default_user_agent()),
        ..Default::default()
    }
}
