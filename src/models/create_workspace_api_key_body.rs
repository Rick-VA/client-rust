/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.6
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkspaceApiKeyBody {
    /// The API Key Name  A descriptive name for the API key.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateWorkspaceApiKeyBody {
    pub fn new(name: String) -> CreateWorkspaceApiKeyBody {
        CreateWorkspaceApiKeyBody {
            name,
        }
    }
}

