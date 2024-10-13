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
pub struct EmailRegisterInput {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
}

impl EmailRegisterInput {
    pub fn new(email: String, password: String, full_name: String) -> EmailRegisterInput {
        EmailRegisterInput {
            email,
            password,
            full_name,
        }
    }
}
