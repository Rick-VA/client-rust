/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateRegistrationFlowWithPasswordMethod : Update Registration Flow with Password Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithPasswordMethod {
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `password` when using the password method.
    #[serde(rename = "method")]
    pub method: String,
    /// Password to sign the user up with
    #[serde(rename = "password")]
    pub password: String,
    /// The identity's traits
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateRegistrationFlowWithPasswordMethod {
    /// Update Registration Flow with Password Method
    pub fn new(method: String, password: String, traits: serde_json::Value) -> UpdateRegistrationFlowWithPasswordMethod {
        UpdateRegistrationFlowWithPasswordMethod {
            csrf_token: None,
            method,
            password,
            traits,
            transient_payload: None,
        }
    }
}

