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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericUsage {
    #[serde(rename = "additional_price")]
    pub additional_price: Box<models::Money>,
    /// IncludedUsage is the number of included items.
    #[serde(rename = "included_usage")]
    pub included_usage: i64,
}

impl GenericUsage {
    pub fn new(additional_price: models::Money, included_usage: i64) -> GenericUsage {
        GenericUsage {
            additional_price: Box::new(additional_price),
            included_usage,
        }
    }
}

