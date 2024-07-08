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
pub struct Workspace {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "subscription_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<Option<String>>,
    #[serde(rename = "subscription_plan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subscription_plan: Option<Option<String>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Workspace {
    pub fn new(created_at: String, id: String, name: String, updated_at: String) -> Workspace {
        Workspace {
            created_at,
            id,
            name,
            subscription_id: None,
            subscription_plan: None,
            updated_at,
        }
    }
}

