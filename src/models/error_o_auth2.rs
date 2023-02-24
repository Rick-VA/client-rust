/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.18
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ErrorOAuth2 : Error



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorOAuth2 {
    /// Error
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Error Debug Information  Only available in dev mode.
    #[serde(rename = "error_debug", skip_serializing_if = "Option::is_none")]
    pub error_debug: Option<String>,
    /// Error Description
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    /// Error Hint  Helps the user identify the error cause.
    #[serde(rename = "error_hint", skip_serializing_if = "Option::is_none")]
    pub error_hint: Option<String>,
    /// HTTP Status Code
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl Default for ErrorOAuth2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorOAuth2 {
    /// Error
    pub fn new() -> ErrorOAuth2 {
        ErrorOAuth2 {
                error: None,
                error_debug: None,
                error_description: None,
                error_hint: None,
                status_code: None,
        }
    }
}


