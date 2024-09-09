use autoi18n_config::{CliConfig, CliConfigOutputFormat};

use crate::{commands::push::PushCommandArguments, error::CliError, DEFAULT_API_HOST};

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

                let translations =
                    serde_json::from_str::<std::collections::HashMap<String, String>>(&raw)?;

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
fn upload_locales(domain: &str, project_id: &str, locales: &[PushLocale]) -> Result<(), CliError> {
    let client = reqwest::blocking::Client::new();

    let url = format!("{domain}/projects/{project_id}/push");

    client.put(url).json(locales).send()?.error_for_status()?;

    Ok(())
}

#[inline]
pub fn run(arguments: &PushCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    let locales = read_locales(&config.output.path, &config.output.format)?;

    if !locales.is_empty() {
        upload_locales(
            if let Some(api_host) = &arguments.api_host {
                api_host
            } else {
                DEFAULT_API_HOST
            },
            &config.project_id,
            &locales,
        )?;
    }

    Ok(())
}
