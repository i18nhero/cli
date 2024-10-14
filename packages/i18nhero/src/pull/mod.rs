use crate::{
    auth::AuthConfig,
    codegen::{
        setup_web_api_configuration,
        web_api::{
            self,
            models::{ExportProjectOutput, FileFormat, PartialExportProjectConfigInput},
        },
    },
    commands::pull::PullCommandArguments,
    config::{CliConfig, CliConfigOutput, CliConfigOutputFormat},
    error::CliError,
    terminal::print_saving_file,
};

#[inline]
async fn fetch_locales(
    web_api_config: &web_api::apis::configuration::Configuration,
    api_key: &str,
    project_id: &str,
    flags: &CliConfigOutput,
) -> Result<Vec<ExportProjectOutput>, CliError> {
    let body = PartialExportProjectConfigInput {
        format: Some(FileFormat::from(flags.format)),
        flat: flags.flat,
        keep_empty_fields: flags.keep_empty_fields,
    };

    web_api::apis::projects_api::pull_project(web_api_config, project_id, api_key, body)
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

    let web_api_config = setup_web_api_configuration(arguments.web_api_host.clone());

    let locales = fetch_locales(
        &web_api_config,
        &auth.api_key,
        &config.project_id,
        &config.output,
    )
    .await?;

    save_locales(config, locales).await
}
