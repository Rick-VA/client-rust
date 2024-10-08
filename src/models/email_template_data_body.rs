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
pub struct EmailTemplateDataBody {
    #[serde(rename = "html")]
    pub html: String,
    #[serde(rename = "plaintext")]
    pub plaintext: String,
}

impl EmailTemplateDataBody {
    pub fn new(html: String, plaintext: String) -> EmailTemplateDataBody {
        EmailTemplateDataBody {
            html,
            plaintext,
        }
    }
}

