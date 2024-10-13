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
pub struct Organization {
    #[serde(rename = "users")]
    pub users: Vec<models::OrganizationUser>,
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "allow_ai_translations")]
    pub allow_ai_translations: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
    #[serde(rename = "deleted_at", deserialize_with = "Option::deserialize")]
    pub deleted_at: Option<String>,
}

impl Organization {
    pub fn new(
        users: Vec<models::OrganizationUser>,
        _id: String,
        title: String,
        allow_ai_translations: bool,
        created_at: String,
        modified_at: String,
        deleted_at: Option<String>,
    ) -> Organization {
        Organization {
            users,
            _id,
            title,
            allow_ai_translations,
            created_at,
            modified_at,
            deleted_at,
        }
    }
}
