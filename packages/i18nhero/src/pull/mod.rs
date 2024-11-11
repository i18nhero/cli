use path_absolutize::Absolutize;

use crate::{
    auth::AuthConfig,
    codegen::{
        public_api::{
            self,
            models::{ExportProjectOutput, FileFormat, PartialExportProjectConfigInput},
        },
        setup_api_configuration,
    },
    commands::pull::PullCommandArguments,
    config::{CliConfig, CliConfigOutput, CliConfigOutputFormat},
    error::CliError,
    terminal::print_pulling_file,
};

#[inline]
async fn fetch_locales(
    api_config: &public_api::apis::configuration::Configuration,
    api_key: &str,
    project_id: &str,
    flags: &CliConfigOutput,
) -> Result<Vec<ExportProjectOutput>, CliError> {
    let body = PartialExportProjectConfigInput {
        format: Some(FileFormat::from(flags.format)),
        flat: flags.flat,
        keep_empty_fields: flags.keep_empty_fields,
    };

    public_api::apis::projects_api::pull_project(api_config, project_id, api_key, body)
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
    let _ = config.create_locale_directory();

    for locale in locales {
        let file_name = locale_file_name(
            &locale.language_code,
            locale.country_code.clone(),
            config.output.format,
        );

        print_pulling_file(&file_name);

        tokio::fs::write(
            config.output.path.join(file_name),
            format!("{}\n", locale.content.trim()),
        )
        .await
        .map_err(CliError::LocaleSave)?;
    }

    Ok(())
}

fn check_if_dirty_repository(path: &std::path::Path) -> Result<(), CliError> {
    let locale_dir = path
        .absolutize()
        .map_or_else(|_| path.canonicalize(), |p| Ok(p.into_owned()))
        .map_err(CliError::Io)?;

    if let Ok(repo) = git2::Repository::discover(path) {
        let mut repo_opts = git2::StatusOptions::new();
        repo_opts.include_ignored(false);
        repo_opts.include_untracked(true);

        let mut dirty_files = Vec::new();

        for status in repo
            .statuses(Some(&mut repo_opts))
            .map_err(CliError::GenericGit)?
            .iter()
        {
            if let Some(path) = status.path().map(std::path::PathBuf::from) {
                let dirty = matches!(
                    status.status(),
                    git2::Status::WT_NEW
                        | git2::Status::WT_MODIFIED
                        | git2::Status::WT_DELETED
                        | git2::Status::WT_RENAMED
                        | git2::Status::WT_TYPECHANGE
                        // NOTE: should this be removed?
                        | git2::Status::CONFLICTED
                );

                if dirty {
                    let is_inside_locale_directory = path
                        .absolutize()
                        .map_or_else(|_| path.canonicalize(), |p| Ok(p.into_owned()))
                        .is_ok_and(|p| p.starts_with(&locale_dir));

                    if is_inside_locale_directory {
                        dirty_files.push(path);
                    }
                }
            }
        }

        if !dirty_files.is_empty() {
            return Err(CliError::PullDirtyRepository(dirty_files));
        }
    }

    Ok(())
}

#[inline]
pub async fn run(arguments: &PullCommandArguments, config: &CliConfig) -> Result<(), CliError> {
    if config.project_id.is_empty() {
        return Err(CliError::MissingProjectId);
    }

    let _ = config.create_locale_directory();

    if !arguments.allow_dirty {
        check_if_dirty_repository(&config.output.path)?;
    }

    let auth = if let Some(api_key) = &arguments.api_key {
        api_key.to_owned()
    } else {
        AuthConfig::load()?.api_key
    };

    let api_config = setup_api_configuration(arguments.api_host.clone());

    let locales = fetch_locales(&api_config, &auth, &config.project_id, &config.output).await?;

    save_locales(config, locales).await
}
