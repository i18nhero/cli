use crate::{
    commands::push::PushCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    DEFAULT_WEB_API_HOST,
};

#[derive(Debug, serde::Serialize)]
struct PushLocale {
    file_name: String,

    translations: std::collections::HashMap<String, String>,
}

#[inline]
fn read_locales(
    folder: &std::path::Path,
    file_format: &CliConfigOutputFormat,
) -> Result<Vec<PushLocale>, CliError> {
    let expected_file_ext = file_format.to_file_ext();

    let mut locales = Vec::new();

    for entry in (std::fs::read_dir(folder)?).flatten() {
        let p = entry.path();

        if p.is_file() && p.extension().is_some_and(|ext| ext == expected_file_ext) {
            if let Some(stem) = p.file_stem() {
                let raw = std::fs::read_to_string(&p)?;

                let translations = crate::generators::parse_input(file_format, &raw)?;

                let locale = PushLocale {
                    file_name: stem.to_string_lossy().to_string(),
                    translations,
                };

                locales.push(locale);
            }
        }
    }

    Ok(locales)
}

#[inline]
async fn upload_locales(
    host: &str,
    project_id: &str,
    locales: &[PushLocale],
) -> Result<(), CliError> {
    let client = reqwest::Client::new();

    let url = format!("{host}/projects/{project_id}/push");

    client
        .put(url)
        .json(locales)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[inline]
pub async fn run(arguments: &PushCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    let locales = read_locales(&config.output.path, &config.output.format)?;

    if !locales.is_empty() {
        upload_locales(
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
