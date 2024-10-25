use anyhow::{Ok, Result};

mod cargo;
mod command_help;
mod readme_tooling;
mod schema;

fn main() -> Result<()> {
    schema::generate()?;

    command_help::generate()?;

    Ok(())
}
