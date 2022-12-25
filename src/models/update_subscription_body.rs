/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateSubscriptionBody : Update Subscription Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubscriptionBody {
    #[serde(rename = "plan_or_price")]
    pub plan_or_price: String,
    #[serde(rename = "return_to", skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
}


impl UpdateSubscriptionBody {
    /// Update Subscription Request Body
    pub fn new(plan_or_price: String) -> UpdateSubscriptionBody {
        UpdateSubscriptionBody {
                plan_or_price,
                return_to: None,
        }
    }
}


