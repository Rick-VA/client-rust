/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.180
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpandTree {
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::ExpandTree>>,
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "subject_set", skip_serializing_if = "Option::is_none")]
    pub subject_set: Option<Box<crate::models::SubjectSet>>,
    #[serde(rename = "type")]
    pub _type: Type,
}

impl ExpandTree {
    pub fn new(_type: Type) -> ExpandTree {
        ExpandTree {
            children: None,
            subject_id: None,
            subject_set: None,
            _type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "union")]
    Union,
    #[serde(rename = "exclusion")]
    Exclusion,
    #[serde(rename = "intersection")]
    Intersection,
    #[serde(rename = "leaf")]
    Leaf,
}

