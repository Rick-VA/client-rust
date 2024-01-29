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
pub struct SessionActivityDatapoint {
    /// Country of the events
    #[serde(rename = "country")]
    pub country: String,
    /// Number of events that failed in the given timeframe
    #[serde(rename = "failed")]
    pub failed: i64,
    /// Number of events that succeeded in the given timeframe
    #[serde(rename = "succeeded")]
    pub succeeded: i64,
}


impl SessionActivityDatapoint {
    pub fn new(country: String, failed: i64, succeeded: i64) -> SessionActivityDatapoint {
        SessionActivityDatapoint {
                country,
                failed,
                succeeded,
        }
    }
}


