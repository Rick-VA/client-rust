/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IdentitySchemaContainer : An Identity JSON Schema Container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitySchemaContainer {
    /// The ID of the Identity JSON Schema
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The actual Identity JSON Schema
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
}

impl IdentitySchemaContainer {
    /// An Identity JSON Schema Container
    pub fn new() -> IdentitySchemaContainer {
        IdentitySchemaContainer {
            id: None,
            schema: None,
        }
    }
}

