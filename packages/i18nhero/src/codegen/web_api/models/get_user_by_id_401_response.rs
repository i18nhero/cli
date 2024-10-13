/*
 * @i18nhero/web-api
 *
 * Api for i18nhero.com
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::codegen::web_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserById401Response {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
}

impl GetUserById401Response {
    pub fn new(message: String, status_code: f64) -> GetUserById401Response {
        GetUserById401Response {
            message,
            status_code,
        }
    }
}
