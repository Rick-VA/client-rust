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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributesCountDatapoint {
    /// Count of the attribute value for given key
    #[serde(rename = "count")]
    pub count: i64,
    /// Name of the attribute value for given key
    #[serde(rename = "name")]
    pub name: String,
}

impl AttributesCountDatapoint {
    pub fn new(count: i64, name: String) -> AttributesCountDatapoint {
        AttributesCountDatapoint {
            count,
            name,
        }
    }
}

