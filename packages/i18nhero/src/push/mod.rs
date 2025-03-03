use crate::{
    auth::AuthConfig,
    codegen::{
        public_api::{
            self,
            models::{FileFormat, PushLocaleInput, PushLocaleInputFile},
        },
        setup_api_configuration,
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
    let output_file_format = FileFormat::from(file_format);

    let mut locales = Vec::new();

    for entry in (std::fs::read_dir(folder).map_err(CliError::LocaleRead)?).flatten() {
        let p = entry.path();

        if p.is_file() && file_format.is_file_extension_match(p.extension()) {
            if let Some(stem) = p.file_stem() {
                let raw = std::fs::read_to_string(&p).map_err(CliError::LocaleRead)?;

                let locale = PushLocaleInputFile {
                    file_name: stem.to_string_lossy().to_string(),
                    file_format: output_file_format,
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
    api_config: &public_api::apis::configuration::Configuration,
    api_key: &str,
    project_id: &str,
    locales: Vec<PushLocaleInputFile>,
) -> Result<public_api::models::PushLocaleResult, CliError> {
    public_api::apis::projects_api::push_locales_to_project(
        api_config,
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

    let api_config = setup_api_configuration(arguments.api_host.clone());

    if locales.is_empty() {
        // TODO: convert to error?
        print_no_locales_to_push();
    } else {
        let auth = if let Some(api_key) = &arguments.api_key {
            api_key.to_owned()
        } else {
            AuthConfig::load()?.api_key
        };

        let locale_count = locales.len();

        upload_locales(&api_config, &auth, &config.project_id, locales).await?;

        print_pushed_locales(locale_count);
    }

    Ok(())
}
