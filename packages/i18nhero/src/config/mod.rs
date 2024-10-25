use json_comments::{CommentSettings, StripComments};

use crate::{codegen::web_api::models::FileFormat, error::CliError};

pub const CONFIG_PATH: &str = "i18nhero.json";

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Copy, schemars::JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum CliConfigOutputFormat {
    #[default]
    #[serde(rename = "json")]
    Json,

    #[serde(rename = "yaml")]
    Yaml,
}

impl CliConfigOutputFormat {
    #[inline]
    pub const fn file_extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Yaml => "yml",
        }
    }

    #[inline]
    pub fn is_file_extension_match(self, ext: Option<&std::ffi::OsStr>) -> bool {
        match self {
            Self::Json => ext.is_some_and(|inner| inner.eq_ignore_ascii_case("json")),
            Self::Yaml => ext.is_some_and(|inner| {
                matches!(
                    inner.to_string_lossy().to_lowercase().as_ref(),
                    "yaml" | "yml"
                )
            }),
        }
    }
}

impl std::fmt::Display for CliConfigOutputFormat {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => f.write_str("json"),
            Self::Yaml => f.write_str("yaml"),
        }
    }
}

impl From<CliConfigOutputFormat> for FileFormat {
    #[inline]
    fn from(value: CliConfigOutputFormat) -> Self {
        match value {
            CliConfigOutputFormat::Json => Self::Json,
            CliConfigOutputFormat::Yaml => Self::Yaml,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct CliConfigOutput {
    /// Defines where locale files should be downloaded to, and uploaded from.
    #[serde(default = "CliConfigOutput::default_path")]
    pub path: std::path::PathBuf,

    /// Defines the file format used when uploading and downloading locale files.
    #[serde(default = "CliConfigOutputFormat::default")]
    pub format: CliConfigOutputFormat,

    /// Defines whether identifiers that are missing translations should be downloaded.
    #[serde(default = "CliConfigOutput::default_keep_empty_fields")]
    pub keep_empty_fields: Option<bool>,

    /// Defines whether the locale files should be a flat `string <-> string` map or a multi layered map.
    ///
    /// A point (`.`) in the identifier name is used to define multi-layered keys when `flat` is set to false.
    ///
    /// The identifier `pages.dashboard.title` will be expanded to the following:
    ///
    /// ```json
    /// {
    ///   "pages": {
    ///     "dashboard": {
    ///       "title": ""
    ///     }
    ///   }
    /// }
    /// ```
    ///
    #[serde(default = "CliConfigOutput::default_flat")]
    pub flat: Option<bool>,
}

impl CliConfigOutput {
    #[inline]
    pub fn default_path() -> std::path::PathBuf {
        std::path::PathBuf::from("lang")
    }

    #[inline]
    const fn default_keep_empty_fields() -> Option<bool> {
        Some(false)
    }

    #[inline]
    const fn default_flat() -> Option<bool> {
        Some(false)
    }
}

impl Default for CliConfigOutput {
    #[inline]
    fn default() -> Self {
        Self {
            path: Self::default_path(),
            format: CliConfigOutputFormat::default(),
            keep_empty_fields: Self::default_keep_empty_fields(),
            flat: Self::default_flat(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct CliConfig {
    #[serde(rename = "$schema", default = "CliConfig::default_schema_location")]
    pub schema: String,

    /// Used to define the linked project.
    pub project_id: String,

    /// Configuration for downloading and uploading locale files.
    pub output: CliConfigOutput,
}

impl Default for CliConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: Self::default_schema_location(),
            output: CliConfigOutput::default(),
            project_id: String::new(),
        }
    }
}

impl CliConfig {
    #[inline]
    pub fn new(
        project_id: String,
        output_path: std::path::PathBuf,
        file_format: CliConfigOutputFormat,
    ) -> Self {
        Self {
            schema: Self::default_schema_location(),
            project_id,
            output: CliConfigOutput {
                path: output_path,
                format: file_format,
                flat: CliConfigOutput::default_flat(),
                keep_empty_fields: CliConfigOutput::default_keep_empty_fields(),
            },
        }
    }

    #[inline]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, CliError> {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let stripped =
                    StripComments::with_settings(CommentSettings::c_style(), content.as_bytes());

                serde_json::from_reader(stripped).map_err(CliError::ConfigParse)
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    Err(CliError::ConfigNotFound)
                } else {
                    Err(CliError::ConfigLoad(error))
                }
            }
        }
    }

    #[inline]
    pub fn default_schema_location() -> String {
        let package_version = env!("CARGO_PKG_VERSION");

        format!(
            "https://raw.githubusercontent.com/i18nhero/cli/main/schemas/v{package_version}/i18nhero.schema.json"
        )
    }

    #[inline]
    pub fn create_locale_directory(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.output.path)
    }
}

#[cfg(test)]
mod test_config {
    use crate::config::CliConfig;

    #[test]
    fn config_should_be_serializable() {
        let config = CliConfig::default();

        let json = serde_json::to_string_pretty(&config).expect("it to be serializable");

        let file = tempfile::Builder::new()
            .suffix(".json")
            .tempfile()
            .expect("it to create file");

        std::fs::write(file.path(), json).expect("it to write to file");

        let loaded = CliConfig::load(file.path()).expect("it to be parsed");

        assert_eq!(config, loaded);
    }

    #[test]
    fn json_schema_should_be_serializable() {
        serde_json::to_string_pretty(&schemars::schema_for!(CliConfig))
            .expect("it to be serializable");
    }
}
