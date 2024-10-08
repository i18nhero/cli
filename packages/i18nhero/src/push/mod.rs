use crate::{
    auth::AuthConfig,
    commands::push::PushCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    DEFAULT_WEB_API_HOST,
};

#[derive(Debug, serde::Serialize)]
struct PushLocale {
    file_name: String,

    file_format: String,

    content: String,
}

#[inline]
fn read_locales(
    folder: &std::path::Path,
    file_format: &CliConfigOutputFormat,
) -> Result<Vec<PushLocale>, CliError> {
    let expected_file_ext = file_format.to_file_ext();

    let mut locales = Vec::new();

    // TODO: log warning?
    let _ = std::fs::create_dir_all(folder);

    for entry in (std::fs::read_dir(folder).map_err(CliError::LocaleRead)?).flatten() {
        let p = entry.path();

        if p.is_file() && p.extension().is_some_and(|ext| ext == expected_file_ext) {
            if let Some(stem) = p.file_stem() {
                let raw = std::fs::read_to_string(&p).map_err(CliError::LocaleRead)?;

                let locale = PushLocale {
                    file_name: stem.to_string_lossy().to_string(),
                    file_format: expected_file_ext.to_string(),
                    content: raw,
                };

                locales.push(locale);
            }
        }
    }

    Ok(locales)
}

#[inline]
async fn upload_locales(
    api_key: &str,
    host: &str,
    project_id: &str,
    locales: &[PushLocale],
) -> Result<(), CliError> {
    let client = reqwest::Client::new();

    client
        .put(format!("{host}/projects/{project_id}/push"))
        .header("x-api-key", api_key)
        .json(locales)
        .send()
        .await
        .map_err(CliError::PushLocaleHttp)?
        .error_for_status()
        .map_err(CliError::PushLocaleHttp)?;

    Ok(())
}

#[inline]
pub async fn run(arguments: &PushCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    let locales = read_locales(&config.output.path, &config.output.format)?;

    if !locales.is_empty() {
        let auth = AuthConfig::load()?;

        upload_locales(
            &auth.api_key,
            arguments
                .api_host
                .as_ref()
                .map_or(DEFAULT_WEB_API_HOST, |api_host| api_host),
            &config.project_id,
            &locales,
        )
        .await?;
    }

    Ok(())
}
