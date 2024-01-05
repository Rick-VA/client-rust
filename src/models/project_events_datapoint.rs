/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.9
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectEventsDatapoint {
    /// Event attributes with details
    #[serde(rename = "attributes")]
    pub attributes: Vec<crate::models::Attribute>,
    /// Name of the event
    #[serde(rename = "name")]
    pub name: String,
    /// Time of occurence
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}


impl ProjectEventsDatapoint {
    pub fn new(attributes: Vec<crate::models::Attribute>, name: String, timestamp: String) -> ProjectEventsDatapoint {
        ProjectEventsDatapoint {
                attributes,
                name,
                timestamp,
        }
    }
}


