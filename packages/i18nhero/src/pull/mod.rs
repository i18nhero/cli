use crate::{
    commands::pull::PullCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    generators,
    terminal::print_saving_file,
    DEFAULT_WEB_API_HOST,
};

#[derive(Debug, serde::Deserialize)]
struct PullLocale {
    language_code: String,

    country_code: Option<String>,

    translations: std::collections::BTreeMap<String, String>,
}

#[inline]
async fn fetch_locales(host: &str, project_id: &str) -> Result<Vec<PullLocale>, CliError> {
    let url = format!("{host}/projects/{project_id}/pull");

    reqwest::get(url)
        .await?
        .error_for_status()?
        .json::<Vec<PullLocale>>()
        .await
        .map_err(CliError::Reqwest)
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

        let contents = generators::stringify(&config.output.format, &locale.translations)?;

        tokio::fs::write(
            config.output.path.join(file_name),
            format!("{}\n", contents.trim()),
        )
        .await?;
    }

    Ok(())
}

#[inline]
pub async fn run(arguments: &PullCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    if config.project_id.is_empty() {
        return Err(CliError::MissingProjectId);
    }

    let mut locales = fetch_locales(
        arguments
            .api_host
            .as_ref()
            .map_or(DEFAULT_WEB_API_HOST, |api_host| api_host),
        &config.project_id,
    )
    .await?;

    // TODO: move to api
    if !config.output.save_missing_values {
        for locale in &mut locales {
            locale.translations.retain(|_, v| !v.is_empty());
        }
    }

    save_locales(config, locales).await
}
