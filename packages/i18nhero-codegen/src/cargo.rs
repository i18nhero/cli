use std::str::FromStr;

use anyhow::{Ok, Result};

#[derive(serde::Deserialize)]
struct Package {
    version: String,
}

#[derive(serde::Deserialize)]
struct Workspace {
    package: Package,
}

#[derive(serde::Deserialize)]
struct Cargo {
    workspace: Workspace,
}

pub fn get_package_version() -> Result<String> {
    let p = std::path::PathBuf::from_str("./Cargo.toml")?;

    let file = std::fs::read_to_string(&p)?;

    let config = toml::from_str::<Cargo>(&file)?;

    Ok(config.workspace.package.version)
}
