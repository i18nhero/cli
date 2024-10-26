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
pub struct User {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
    #[serde(rename = "deleted_at", deserialize_with = "Option::deserialize")]
    pub deleted_at: Option<String>,
}

impl User {
    pub fn new(
        _id: String,
        full_name: String,
        created_at: String,
        modified_at: String,
        deleted_at: Option<String>,
    ) -> User {
        User {
            _id,
            full_name,
            created_at,
            modified_at,
            deleted_at,
        }
    }
}
