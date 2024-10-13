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
pub struct TranslateAiInput {
    #[serde(rename = "from")]
    pub from: String,
}

impl TranslateAiInput {
    pub fn new(from: String) -> TranslateAiInput {
        TranslateAiInput { from }
    }
}
