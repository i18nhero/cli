use crate::{
    auth::AuthConfig,
    codegen::{
        setup_web_api_configuration,
        web_api::{
            self,
            models::{FileFormat, PushLocaleInput, PushLocaleInputFile},
        },
    },
    commands::push::PushCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    terminal::{print_no_locales_to_push, print_pushed_locales},
};

#[inline]
fn read_locales(
    folder: &std::path::Path,
    file_format: CliConfigOutputFormat,
) -> Result<Vec<PushLocaleInputFile>, CliError> {
    let expected_file_ext = file_format.file_extension();

    let file_format = FileFormat::from(file_format);

    let mut locales = Vec::new();

    for entry in (std::fs::read_dir(folder).map_err(CliError::LocaleRead)?).flatten() {
        let p = entry.path();

        if p.is_file() && p.extension().is_some_and(|ext| ext == expected_file_ext) {
            if let Some(stem) = p.file_stem() {
                let raw = std::fs::read_to_string(&p).map_err(CliError::LocaleRead)?;

                let locale = PushLocaleInputFile {
                    file_name: stem.to_string_lossy().to_string(),
                    file_format,
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
    web_api_config: &web_api::apis::configuration::Configuration,
    api_key: &str,
    project_id: &str,
    locales: Vec<PushLocaleInputFile>,
) -> Result<web_api::models::PushLocaleResult, CliError> {
    web_api::apis::projects_api::push_locales_to_project(
        web_api_config,
        project_id,
        api_key,
        PushLocaleInput { files: locales },
    )
    .await
    .map_err(CliError::PushLocaleHttp)
}

#[inline]
pub async fn run(arguments: &PushCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    let _ = config.create_locale_directory();

    let locales = read_locales(&config.output.path, config.output.format)?;

    let web_api_config = setup_web_api_configuration(arguments.web_api_host.clone());

    if locales.is_empty() {
        // TODO: convert to error?
        print_no_locales_to_push();
    } else {
        let auth = AuthConfig::load()?;

        let locale_count = locales.len();

        upload_locales(&web_api_config, &auth.api_key, &config.project_id, locales).await?;

        print_pushed_locales(locale_count);
    }

    Ok(())
}
