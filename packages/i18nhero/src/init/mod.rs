use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    auth::AuthConfig,
    commands::init::InitCommandArguments,
    config::{CliConfig, CONFIG_PATH},
    error::CliError,
    terminal::print_configuration_file_created,
    DEFAULT_API_HOST,
};

#[derive(serde::Deserialize, Debug)]
struct Organization {
    #[serde(rename = "_id")]
    id: String,

    title: String,
}

#[inline]
async fn get_organizations(host: &str, api_key: &str) -> Result<Vec<Organization>, CliError> {
    let http_client = reqwest::Client::new();

    http_client
        .get(format!("{host}/organizations"))
        .header("x-api-key", api_key)
        .send()
        .await
        .map_err(CliError::GetOrganizations)?
        .error_for_status()
        .map_err(CliError::GetOrganizations)?
        .json::<Vec<Organization>>()
        .await
        .map_err(CliError::GetOrganizations)
}

#[inline]
fn select_organization(organizations: &Vec<Organization>) -> usize {
    let mut options = Vec::with_capacity(organizations.len());

    for org in organizations {
        options.push(format!("{} ({})", org.title, org.id));
    }

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Organization")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap()
}

#[derive(Debug, serde::Deserialize)]
struct Project {
    #[serde(rename = "_id")]
    id: String,

    title: String,
}

#[inline]
async fn get_organization_projects(
    host: &str,
    api_key: &str,
    organization_id: &str,
) -> Result<Vec<Project>, CliError> {
    let http_client = reqwest::Client::new();

    http_client
        .get(format!("{host}/organizations/{organization_id}/projects"))
        .header("x-api-key", api_key)
        .send()
        .await
        .map_err(CliError::GetOrganizationProjects)?
        .error_for_status()
        .map_err(CliError::GetOrganizationProjects)?
        .json::<Vec<Project>>()
        .await
        .map_err(CliError::GetOrganizationProjects)
}

#[inline]
fn select_project(projects: &Vec<Project>) -> usize {
    let mut options = Vec::with_capacity(projects.len());

    for project in projects {
        options.push(format!("{} ({})", project.title, project.id));
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

    let host = arguments
        .api_host
        .as_ref()
        .map_or(DEFAULT_API_HOST, |api_host| api_host);

    let organizations = get_organizations(host, &auth.api_key).await?;

    if organizations.is_empty() {
        return Err(CliError::NoConnectedOrganizations);
    }

    let organization_index = select_organization(&organizations);

    // we can unwrap here since it can't be out of bounds
    let selected_organization = organizations.get(organization_index).unwrap();

    let projects =
        get_organization_projects(host, &auth.api_key, &selected_organization.id).await?;

    if projects.is_empty() {
        return Err(CliError::NoAvailableProjects((
            selected_organization.title.to_string(),
            selected_organization.id.to_string(),
        )));
    }

    let project_index = select_project(&projects);

    // we can unwrap here since it can't be out of bounds
    let selected_project = projects.get(project_index).unwrap();

    let config = CliConfig::new(selected_project.id.to_string());

    let mut json = serde_json::to_string_pretty(&config).map_err(CliError::ConfigSerialize)?;

    json.push('\n');

    std::fs::write(CONFIG_PATH, json).map_err(CliError::ConfigSave)?;

    print_configuration_file_created();

    Ok(())
}
