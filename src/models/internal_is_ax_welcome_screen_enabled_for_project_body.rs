/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.13.6
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InternalIsAxWelcomeScreenEnabledForProjectBody : Is Account Experience Enabled For Project Request Body
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalIsAxWelcomeScreenEnabledForProjectBody {
    /// Path is the path of the request.
    #[serde(rename = "path")]
    pub path: String,
    /// ProjectSlug is the project's slug.
    #[serde(rename = "project_slug")]
    pub project_slug: String,
}

impl InternalIsAxWelcomeScreenEnabledForProjectBody {
    /// Is Account Experience Enabled For Project Request Body
    pub fn new(path: String, project_slug: String) -> InternalIsAxWelcomeScreenEnabledForProjectBody {
        InternalIsAxWelcomeScreenEnabledForProjectBody {
            path,
            project_slug,
        }
    }
}

