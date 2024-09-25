use dialoguer::{theme::ColorfulTheme, Select};
use i18nhero_config::CliConfig;

use crate::{
    auth::get_api_key, commands::init::InitCommandArguments, config::CONFIG_PATH, error::CliError,
    terminal::print_configuration_file_created, DEFAULT_API_HOST,
};

#[derive(serde::Deserialize, Debug)]
pub struct Organization {
    pub _id: String,

    pub title: String,
}

async fn get_organizations(host: &str, api_key: &str) -> Vec<Organization> {
    let http_client = reqwest::Client::new();

    http_client
        .get(format!("{host}/organizations"))
        .header("x-api-key", api_key)
        .send()
        .await
        .unwrap()
        .json::<Vec<Organization>>()
        .await
        .unwrap()
}

fn select_organization(organizations: &Vec<Organization>) -> usize {
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

#[derive(Debug, serde::Deserialize)]
struct Project {
    _id: String,
    title: String,
}

async fn get_organization_projects(
    host: &str,
    api_key: &str,
    organization_id: &str,
) -> Vec<Project> {
    let http_client = reqwest::Client::new();

    http_client
        .get(format!("{host}/organizations/{organization_id}/projects"))
        .header("x-api-key", api_key)
        .send()
        .await
        .unwrap()
        .json::<Vec<Project>>()
        .await
        .unwrap()
}

fn select_project(projects: &Vec<Project>) -> usize {
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
    if !arguments.overwrite && std::fs::exists(CONFIG_PATH)? {
        return Err(CliError::ConfigAlreadyExists);
    }

    let api_key = get_api_key().await?;

    let host = arguments
        .api_host
        .as_ref()
        .map_or(DEFAULT_API_HOST, |api_host| api_host);

    let organizations = get_organizations(host, &api_key).await;

    if organizations.is_empty() {
        return Err(CliError::NoConnectedOrganizations);
    }

    let organization_index = select_organization(&organizations);

    // we can unwrap here since it can't be out of bounds
    let selected_organization = organizations.get(organization_index).unwrap();

    let projects = get_organization_projects(host, &api_key, &selected_organization._id).await;

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

    let mut json = serde_json::to_string_pretty(&config)?;

    json.push('\n');

    std::fs::write(CONFIG_PATH, json)?;

    print_configuration_file_created();

    Ok(())
}
