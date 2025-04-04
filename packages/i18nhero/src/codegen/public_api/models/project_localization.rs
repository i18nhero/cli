/*
 * @i18nhero/public-api
 *
 * Public api for i18nhero.com
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::codegen::public_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectLocalization {
    #[serde(rename = "language_code")]
    pub language_code: String,
    #[serde(rename = "country_code", deserialize_with = "Option::deserialize")]
    pub country_code: Option<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl ProjectLocalization {
    pub fn new(
        language_code: String,
        country_code: Option<String>,
        enabled: bool,
    ) -> ProjectLocalization {
        ProjectLocalization {
            language_code,
            country_code,
            enabled,
        }
    }
}
