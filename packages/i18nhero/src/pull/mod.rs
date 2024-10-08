use crate::{
    auth::AuthConfig,
    commands::pull::PullCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    terminal::print_saving_file,
    DEFAULT_WEB_API_HOST,
};

#[derive(Debug, serde::Deserialize)]
struct PullLocale {
    language_code: String,

    country_code: Option<String>,

    content: String,
}

#[inline]
async fn fetch_locales(
    api_key: &str,
    host: &str,
    project_id: &str,
) -> Result<Vec<PullLocale>, CliError> {
    let client = reqwest::Client::new();

    client
        .get(format!("{host}/projects/{project_id}/pull"))
        .header("x-api-key", api_key)
        .send()
        .await
        .map_err(CliError::PullLocaleHttp)?
        .error_for_status()
        .map_err(CliError::PullLocaleHttp)?
        .json::<Vec<PullLocale>>()
        .await
        .map_err(CliError::PullLocaleHttp)
}

#[inline]
fn locale_file_name(
    language: &str,
    country: Option<String>,
    output_format: &CliConfigOutputFormat,
) -> String {
    let locale = country.map_or_else(
        || language.to_owned(),
        |country| format!("{language}_{country}"),
    );

    format!("{locale}.{}", output_format.to_file_ext())
}

#[inline]
async fn save_locales(config: &CliConfig, locales: Vec<PullLocale>) -> Result<(), CliError> {
    let _ = std::fs::create_dir_all(&config.output.path);

    for locale in locales {
        let file_name = locale_file_name(
            &locale.language_code,
            locale.country_code.clone(),
            &config.output.format,
        );

        print_saving_file(&file_name);

        tokio::fs::write(
            config.output.path.join(file_name),
            format!("{}\n", locale.content.trim()),
        )
        .await
        .map_err(CliError::LocaleSave)?;
    }

    Ok(())
}

#[inline]
pub async fn run(arguments: &PullCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    if config.project_id.is_empty() {
        return Err(CliError::MissingProjectId);
    }

    let auth = AuthConfig::load()?;

    let locales = fetch_locales(
        &auth.api_key,
        arguments
            .api_host
            .as_ref()
            .map_or(DEFAULT_WEB_API_HOST, |api_host| api_host),
        &config.project_id,
    )
    .await?;

    save_locales(config, locales).await
}
