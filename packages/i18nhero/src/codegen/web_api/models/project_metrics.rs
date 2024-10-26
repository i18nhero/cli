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
pub struct ProjectMetrics {
    #[serde(rename = "locales")]
    pub locales: std::collections::HashMap<String, f64>,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "identifier_count")]
    pub identifier_count: f64,
}

impl ProjectMetrics {
    pub fn new(
        locales: std::collections::HashMap<String, f64>,
        project_id: String,
        identifier_count: f64,
    ) -> ProjectMetrics {
        ProjectMetrics {
            locales,
            project_id,
            identifier_count,
        }
    }
}
