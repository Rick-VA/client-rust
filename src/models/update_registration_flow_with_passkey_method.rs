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

/// UpdateRegistrationFlowWithPasskeyMethod : Update Registration Flow with Passkey Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithPasskeyMethod {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to \"passkey\" when trying to add, update, or remove a Passkey.
    #[serde(rename = "method")]
    pub method: String,
    /// Register a WebAuthn Security Key  It is expected that the JSON returned by the WebAuthn registration process is included here.
    #[serde(rename = "passkey_register", skip_serializing_if = "Option::is_none")]
    pub passkey_register: Option<String>,
    /// The identity's traits
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateRegistrationFlowWithPasskeyMethod {
    /// Update Registration Flow with Passkey Method
    pub fn new(method: String, traits: serde_json::Value) -> UpdateRegistrationFlowWithPasskeyMethod {
        UpdateRegistrationFlowWithPasskeyMethod {
            csrf_token: None,
            method,
            passkey_register: None,
            traits,
            transient_payload: None,
        }
    }
}

