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
pub struct AttributeFilter {
    #[serde(rename = "attribute", skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionEnum>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Default for AttributeFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeFilter {
    pub fn new() -> AttributeFilter {
        AttributeFilter {
                attribute: None,
                condition: None,
                value: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConditionEnum {
    #[serde(rename = "equals")]
    Equals,
    #[serde(rename = "not_equals")]
    NotEquals,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "not_contains")]
    NotContains,
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "not_regex")]
    NotRegex,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "not_set")]
    NotSet,
}

