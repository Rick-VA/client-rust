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
pub struct BillingPeriodBucket {
    #[serde(rename = "base_invoices", skip_serializing_if = "Option::is_none")]
    pub base_invoices: Option<Vec<models::Invoice>>,
    #[serde(rename = "billing_period", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<Box<models::TimeInterval>>,
    #[serde(rename = "usage_invoice", skip_serializing_if = "Option::is_none")]
    pub usage_invoice: Option<Box<models::Invoice>>,
}

impl BillingPeriodBucket {
    pub fn new() -> BillingPeriodBucket {
        BillingPeriodBucket {
            base_invoices: None,
            billing_period: None,
            usage_invoice: None,
        }
    }
}

