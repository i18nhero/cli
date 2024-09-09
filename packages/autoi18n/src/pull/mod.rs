use autoi18n_config::{CliConfig, CliConfigOutputFormat};

use crate::{commands::pull::PullCommandArguments, error::CliError, DEFAULT_API_HOST};

#[derive(Debug, serde::Deserialize)]
struct PullLocale {
    language_code: String,

    country_code: Option<String>,

    translations: std::collections::BTreeMap<String, String>,
}

#[inline]
fn fetch_locales(host: &str, project_id: &str) -> Result<Vec<PullLocale>, CliError> {
    let url = format!("{host}/projects/{project_id}/pull");

    reqwest::blocking::get(url)?
        .error_for_status()?
        .json::<Vec<PullLocale>>()
        .map_err(CliError::from)
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
fn save_locales(config: &CliConfig, locales: Vec<PullLocale>) -> Result<(), CliError> {
    let _ = std::fs::create_dir_all(&config.output.path);

    for locale in locales {
        let file_name = locale_file_name(
            &locale.language_code,
            locale.country_code.clone(),
            &config.output.format,
        );

        println!("Saving {file_name}");

        let contents = match config.output.format {
            CliConfigOutputFormat::Json => serde_json::to_string_pretty(&locale.translations)?,
        };

        std::fs::write(
            config.output.path.join(file_name),
            format!("{}\n", contents.trim()),
        )?;
    }

    Ok(())
}

#[inline]
pub fn run(arguments: &PullCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    if config.project_id.is_empty() {
        return Err(CliError::MissingProjectId);
    }

    let locales = fetch_locales(
        if let Some(api_host) = &arguments.api_host {
            api_host
        } else {
            DEFAULT_API_HOST
        },
        &config.project_id,
    )?;

    save_locales(config, locales)
}
