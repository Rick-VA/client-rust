/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GetProjectEventsBody : Body of the getProjectEvents endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProjectEventsBody {
    /// The event name to query for
    #[serde(rename = "event_name", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// Event attribute filters
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<models::AttributeFilter>>,
    /// The start RFC3339 date of the time window
    #[serde(rename = "from")]
    pub from: String,
    /// Maximum number of events to return
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Pagination token to fetch next page, empty if first page
    #[serde(rename = "page_token", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// The end RFC3339 date of the time window
    #[serde(rename = "to")]
    pub to: String,
}

impl GetProjectEventsBody {
    /// Body of the getProjectEvents endpoint
    pub fn new(from: String, to: String) -> GetProjectEventsBody {
        GetProjectEventsBody {
            event_name: None,
            filters: None,
            from,
            page_size: None,
            page_token: None,
            to,
        }
    }
}

