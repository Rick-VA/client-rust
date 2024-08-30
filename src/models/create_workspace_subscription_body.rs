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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkspaceSubscriptionBody {
    ///  usd USD eur Euro
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<CurrencyEnum>,
    ///  monthly Monthly yearly Yearly
    #[serde(rename = "interval")]
    pub interval: IntervalEnum,
    #[serde(rename = "plan")]
    pub plan: String,
    #[serde(rename = "return_to", skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
}

impl CreateWorkspaceSubscriptionBody {
    pub fn new(interval: IntervalEnum, plan: String) -> CreateWorkspaceSubscriptionBody {
        CreateWorkspaceSubscriptionBody {
            currency: None,
            interval,
            plan,
            return_to: None,
        }
    }
}
///  usd USD eur Euro
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CurrencyEnum {
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "eur")]
    Eur,
}

impl Default for CurrencyEnum {
    fn default() -> CurrencyEnum {
        Self::Usd
    }
}
///  monthly Monthly yearly Yearly
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntervalEnum {
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}

impl Default for IntervalEnum {
    fn default() -> IntervalEnum {
        Self::Monthly
    }
}

