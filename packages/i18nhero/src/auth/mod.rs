use crate::error::CliError;

pub async fn get_api_key() -> Result<String, CliError> {
    // TODO:

    let api_key = "66d7828c3c19bf6163ac150a".to_owned();

    Ok(api_key)
}
