/*
 * @i18nhero/public-api
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::codegen::cli_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "title")]
    pub title: String,
}

impl Organization {
    pub fn new(_id: String, title: String) -> Organization {
        Organization { _id, title }
    }
}
