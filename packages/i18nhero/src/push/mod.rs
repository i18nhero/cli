use crate::{
    auth::AuthConfig,
    codegen::web_api::{
        self,
        models::{FileFormat, PushLocaleInput, PushLocaleInputFile},
    },
    commands::push::PushCommandArguments,
    config::{CliConfig, CliConfigOutputFormat},
    error::CliError,
    DEFAULT_WEB_API_HOST,
};

#[inline]
fn read_locales(
    folder: &std::path::Path,
    file_format: CliConfigOutputFormat,
) -> Result<Vec<PushLocaleInputFile>, CliError> {
    let expected_file_ext = file_format.file_extension();

    let file_format = FileFormat::from(file_format);

    let mut locales = Vec::new();

    // TODO: log warning?
    let _ = std::fs::create_dir_all(folder);

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
    api_key: &str,
    host: &str,
    project_id: &str,
    locales: Vec<PushLocaleInputFile>,
) -> Result<web_api::models::PushLocaleResult, CliError> {
    let conf = web_api::apis::configuration::Configuration {
        base_path: host.to_owned(),
        ..Default::default()
    };

    web_api::apis::projects_api::push_locales_to_project(
        &conf,
        project_id,
        api_key,
        PushLocaleInput { files: locales },
    )
    .await
    .map_err(CliError::PushLocaleHttp)
}

#[inline]
pub async fn run(arguments: &PushCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    let locales = read_locales(&config.output.path, config.output.format)?;

    let web_api_host = arguments
        .web_api_host
        .as_ref()
        .map_or(DEFAULT_WEB_API_HOST, |host| host);

    if !locales.is_empty() {
        let auth = AuthConfig::load()?;

        upload_locales(&auth.api_key, web_api_host, &config.project_id, locales).await?;
    }

    Ok(())
}
