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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectWebhookEvent {
    #[serde(rename = "key:created")]
    Created,
    #[serde(rename = "key:bulk:created")]
    BulkColonCreated,
    #[serde(rename = "key:updated")]
    Updated,
    #[serde(rename = "key:bulk:updated")]
    BulkColonUpdated,
    #[serde(rename = "key:deleted")]
    Deleted,
    #[serde(rename = "key:bulk:deleted")]
    BulkColonDeleted,
}

impl std::fmt::Display for ProjectWebhookEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "key:created"),
            Self::BulkColonCreated => write!(f, "key:bulk:created"),
            Self::Updated => write!(f, "key:updated"),
            Self::BulkColonUpdated => write!(f, "key:bulk:updated"),
            Self::Deleted => write!(f, "key:deleted"),
            Self::BulkColonDeleted => write!(f, "key:bulk:deleted"),
        }
    }
}

impl Default for ProjectWebhookEvent {
    fn default() -> ProjectWebhookEvent {
        Self::Created
    }
}
