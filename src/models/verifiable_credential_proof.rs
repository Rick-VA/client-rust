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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerifiableCredentialProof {
    #[serde(rename = "jwt", skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,
    #[serde(rename = "proof_type", skip_serializing_if = "Option::is_none")]
    pub proof_type: Option<String>,
}

impl VerifiableCredentialProof {
    pub fn new() -> VerifiableCredentialProof {
        VerifiableCredentialProof {
            jwt: None,
            proof_type: None,
        }
    }
}

