/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.18
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CreateSubscriptionBody : Create Subscription Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSubscriptionBody {
    #[serde(rename = "plan_or_price")]
    pub plan_or_price: String,
    #[serde(rename = "provision_first_project")]
    pub provision_first_project: String,
    #[serde(rename = "return_to", skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
}


impl CreateSubscriptionBody {
    /// Create Subscription Request Body
    pub fn new(plan_or_price: String, provision_first_project: String) -> CreateSubscriptionBody {
        CreateSubscriptionBody {
                plan_or_price,
                provision_first_project,
                return_to: None,
        }
    }
}


