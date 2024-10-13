use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    auth::AuthConfig,
    codegen,
    commands::init::InitCommandArguments,
    config::{CliConfig, CONFIG_PATH},
    error::CliError,
    terminal::print_configuration_file_created,
    DEFAULT_CLI_API_HOST,
};

#[inline]
async fn get_organizations(
    configuration: &codegen::cli_api::apis::configuration::Configuration,
    api_key: &str,
) -> Result<Vec<codegen::cli_api::models::Organization>, CliError> {
    codegen::cli_api::apis::default_api::get_organizations(configuration, api_key)
        .await
        .map_err(CliError::GetOrganizations)
}

#[inline]
fn select_organization(organizations: &Vec<codegen::cli_api::models::Organization>) -> usize {
    let mut options = Vec::with_capacity(organizations.len());

    for org in organizations {
        options.push(format!("{} ({})", org.title, org._id));
    }

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Organization")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap()
}

#[inline]
async fn get_organization_projects(
    configuration: &codegen::cli_api::apis::configuration::Configuration,
    api_key: &str,
    organization_id: &str,
) -> Result<Vec<codegen::cli_api::models::Project>, CliError> {
    codegen::cli_api::apis::default_api::get_organization_projects(
        configuration,
        api_key,
        organization_id,
    )
    .await
    .map_err(CliError::GetOrganizationProjects)
}

#[inline]
fn select_project(projects: &Vec<codegen::cli_api::models::Project>) -> usize {
    let mut options = Vec::with_capacity(projects.len());

    for project in projects {
        options.push(format!("{} ({})", project.title, project._id));
    }

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap()
}

#[inline]
pub async fn run(arguments: &InitCommandArguments) -> Result<(), CliError> {
    if !arguments.overwrite && std::fs::exists(CONFIG_PATH).map_err(CliError::Io)? {
        return Err(CliError::ConfigAlreadyExists);
    }

    let auth = AuthConfig::load()?;

    let cli_api_host = arguments
        .cli_api_host
        .as_ref()
        .map_or(DEFAULT_CLI_API_HOST, |host| host);

    let cli_api_config = codegen::cli_api::apis::configuration::Configuration {
        base_path: cli_api_host.to_owned(),
        ..Default::default()
    };

    let organizations = get_organizations(&cli_api_config, &auth.api_key).await?;

    if organizations.is_empty() {
        return Err(CliError::NoConnectedOrganizations);
    }

    let organization_index = select_organization(&organizations);

    // we can unwrap here since it can't be out of bounds
    let selected_organization = organizations.get(organization_index).unwrap();

    let projects =
        get_organization_projects(&cli_api_config, &auth.api_key, &selected_organization._id)
            .await?;

    if projects.is_empty() {
        return Err(CliError::NoAvailableProjects((
            selected_organization.title.to_string(),
            selected_organization._id.to_string(),
        )));
    }

    let project_index = select_project(&projects);

    // we can unwrap here since it can't be out of bounds
    let selected_project = projects.get(project_index).unwrap();

    let config = CliConfig::new(selected_project._id.to_string());

    let mut json = serde_json::to_string_pretty(&config).map_err(CliError::ConfigSerialize)?;

    json.push('\n');

    std::fs::write(CONFIG_PATH, json).map_err(CliError::ConfigSave)?;

    print_configuration_file_created();

    Ok(())
}
