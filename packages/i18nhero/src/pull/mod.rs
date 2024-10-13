use crate::{
    auth::AuthConfig,
    codegen::web_api::{
        self,
        models::{ExportProjectOutput, FileFormat, PartialExportProjectConfigInput},
    },
    commands::pull::PullCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    terminal::print_saving_file,
    DEFAULT_WEB_API_HOST,
};

#[inline]
async fn fetch_locales(
    config: &CliConfig,
    api_key: &str,
    host: &str,
    project_id: &str,
) -> Result<Vec<ExportProjectOutput>, CliError> {
    let conf = web_api::apis::configuration::Configuration {
        base_path: host.to_owned(),
        ..Default::default()
    };

    let body = PartialExportProjectConfigInput {
        format: Some(FileFormat::from(config.output.format)),
        flat: config.output.flat,
        keep_empty_fields: config.output.keep_empty_fields,
    };

    web_api::apis::projects_api::pull_project(&conf, project_id, api_key, body)
        .await
        .map_err(CliError::PullLocaleHttp)
}

#[inline]
fn locale_file_name(
    language: &str,
    country: Option<String>,
    output_format: CliConfigOutputFormat,
) -> String {
    let locale = country.map_or_else(
        || language.to_owned(),
        |country| format!("{language}_{country}"),
    );

    format!("{locale}.{}", output_format.file_extension())
}

#[inline]
async fn save_locales(
    config: &CliConfig,
    locales: Vec<ExportProjectOutput>,
) -> Result<(), CliError> {
    let _ = std::fs::create_dir_all(&config.output.path);

    for locale in locales {
        let file_name = locale_file_name(
            &locale.language_code,
            locale.country_code.clone(),
            config.output.format,
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

    let web_api_host = arguments
        .web_api_host
        .as_ref()
        .map_or(DEFAULT_WEB_API_HOST, |host| host);

    let locales = fetch_locales(config, &auth.api_key, web_api_host, &config.project_id).await?;

    save_locales(config, locales).await
}
