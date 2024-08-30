/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ContinueWithVerificationUi : Indicates, that the UI flow could be continued by showing a verification ui
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithVerificationUi {
    /// Action will always be `show_verification_ui` show_verification_ui ContinueWithActionShowVerificationUIString
    #[serde(rename = "action")]
    pub action: ActionEnum,
    #[serde(rename = "flow")]
    pub flow: Box<models::ContinueWithVerificationUiFlow>,
}

impl ContinueWithVerificationUi {
    /// Indicates, that the UI flow could be continued by showing a verification ui
    pub fn new(action: ActionEnum, flow: models::ContinueWithVerificationUiFlow) -> ContinueWithVerificationUi {
        ContinueWithVerificationUi {
            action,
            flow: Box::new(flow),
        }
    }
}
/// Action will always be `show_verification_ui` show_verification_ui ContinueWithActionShowVerificationUIString
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionEnum {
    #[serde(rename = "show_verification_ui")]
    ShowVerificationUi,
}

impl Default for ActionEnum {
    fn default() -> ActionEnum {
        Self::ShowVerificationUi
    }
}

