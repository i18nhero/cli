use crate::codegen::web_api;

#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    PushLocaleHttp(web_api::apis::Error<web_api::apis::projects_api::PushLocalesToProjectError>),
    PullLocaleHttp(web_api::apis::Error<web_api::apis::projects_api::PullProjectError>),
    ConfigAlreadyExists,
    MissingProjectId,
    NoConnectedOrganizations,
    NoAvailableProjects((String, String)),
    ConfigNotFound,
    ConfigLoad(std::io::Error),
    ConfigSave(std::io::Error),
    LocaleSave(std::io::Error),
    LocaleRead(std::io::Error),
    ConfigParse(serde_json::Error),
    ConfigSerialize(serde_json::Error),
    AuthConfigSerialize(serde_json::Error),
    AuthConfigDeserialize(serde_json::Error),
    AuthConfigSave(std::io::Error),
    AuthConfigLoad(std::io::Error),
    GetOrganizations(reqwest::Error),
    GetOrganizationProjects(reqwest::Error),
}

impl std::error::Error for CliError {}

impl core::fmt::Display for CliError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::ConfigAlreadyExists => write!(f, "A configuration file already exists"),
            Self::MissingProjectId => write!(f, "project_id must be set in config"),
            Self::NoConnectedOrganizations => {
                write!(f, "You do not have development access to any organizations")
            }
            Self::NoAvailableProjects((organization_title, organization_id)) => {
                let message = "You do not have development access to any projects for";

                write!(f, "{message} {organization_title} ({organization_id})")
            }
            Self::ConfigNotFound => write!(f, "Config not found"),
            Self::ConfigLoad(e) => write!(f, "Error loading config - {e}"),
            Self::ConfigSave(e) => write!(f, "Error saving config - {e}"),
            Self::ConfigParse(e) => write!(f, "Error parsing config - {e}"),
            Self::ConfigSerialize(e) => write!(f, "Error serializing config - {e}"),
            Self::AuthConfigSerialize(e) => write!(f, "Error serializing auth config - {e}"),
            Self::AuthConfigDeserialize(e) => write!(f, "Error deserializing auth config - {e}"),
            Self::AuthConfigSave(e) => write!(f, "Error saving auth - {e}"),
            Self::AuthConfigLoad(e) => write!(f, "Error loading auth - {e}"),
            Self::GetOrganizations(e) => write!(f, "Error fetching organizations - {e}"),
            Self::GetOrganizationProjects(e) => write!(f, "Error fetching projects - {e}"),
            Self::PushLocaleHttp(e) => write!(f, "Error pushing locales - {e}"),
            Self::PullLocaleHttp(e) => write!(f, "Error pulling locales - {e}"),
            Self::LocaleSave(e) => write!(f, "Error saving locale - {e}"),
            Self::LocaleRead(e) => write!(f, "Error reading locales - {e}"),
        }
    }
}
