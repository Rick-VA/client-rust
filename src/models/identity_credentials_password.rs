/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.1.0-alpha.9
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCredentialsPassword {
    /// HashedPassword is a hash-representation of the password.
    #[serde(rename = "hashed_password", skip_serializing_if = "Option::is_none")]
    pub hashed_password: Option<String>,
}

impl IdentityCredentialsPassword {
    pub fn new() -> IdentityCredentialsPassword {
        IdentityCredentialsPassword {
            hashed_password: None,
        }
    }
}


