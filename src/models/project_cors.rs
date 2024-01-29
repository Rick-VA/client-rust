/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.5.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectCors {
    /// Whether CORS is enabled for this endpoint.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The allowed origins. Use `*` to allow all origins. A wildcard can also be used in the subdomain, i.e. `https://_*.example.com` will allow all origins on all subdomains of `example.com`.
    #[serde(rename = "origins", skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,
}

impl Default for ProjectCors {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectCors {
    pub fn new() -> ProjectCors {
        ProjectCors {
                enabled: None,
                origins: None,
        }
    }
}


