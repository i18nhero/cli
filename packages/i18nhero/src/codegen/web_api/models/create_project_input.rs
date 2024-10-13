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
pub struct CreateProjectInput {
    #[serde(rename = "localizations")]
    pub localizations: Vec<models::CreateProjectInputLocalization>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "is_public")]
    pub is_public: bool,
}

impl CreateProjectInput {
    pub fn new(
        localizations: Vec<models::CreateProjectInputLocalization>,
        title: String,
        organization_id: String,
        description: String,
        is_public: bool,
    ) -> CreateProjectInput {
        CreateProjectInput {
            localizations,
            title,
            organization_id,
            description,
            is_public,
        }
    }
}