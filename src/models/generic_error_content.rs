/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.13.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GenericErrorContent : Error response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericErrorContent {
    /// Debug contains debug information. This is usually not available and has to be enabled.
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<String>,
    /// Name is the error name.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Description contains further information on the nature of the error.
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    /// Message contains the error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Code represents the error status code (404, 403, 401, ...).
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl GenericErrorContent {
    /// Error response
    pub fn new() -> GenericErrorContent {
        GenericErrorContent {
            debug: None,
            error: None,
            error_description: None,
            message: None,
            status_code: None,
        }
    }
}

