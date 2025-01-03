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
pub struct PushLocaleResult {
    #[serde(rename = "inserted")]
    pub inserted: f64,
    #[serde(rename = "modified")]
    pub modified: f64,
}

impl PushLocaleResult {
    pub fn new(inserted: f64, modified: f64) -> PushLocaleResult {
        PushLocaleResult { inserted, modified }
    }
}
