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

/// LoginFlowState : The state represents the state of the login flow.  choose_method: ask the user to choose a method (e.g. login account via email) sent_email: the email has been sent to the user passed_challenge: the request was successful and the login challenge was passed.
/// The state represents the state of the login flow.  choose_method: ask the user to choose a method (e.g. login account via email) sent_email: the email has been sent to the user passed_challenge: the request was successful and the login challenge was passed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoginFlowState {
    #[serde(rename = "choose_method")]
    ChooseMethod,
    #[serde(rename = "sent_email")]
    SentEmail,
    #[serde(rename = "passed_challenge")]
    PassedChallenge,

}

impl std::fmt::Display for LoginFlowState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChooseMethod => write!(f, "choose_method"),
            Self::SentEmail => write!(f, "sent_email"),
            Self::PassedChallenge => write!(f, "passed_challenge"),
        }
    }
}

impl Default for LoginFlowState {
    fn default() -> LoginFlowState {
        Self::ChooseMethod
    }
}

