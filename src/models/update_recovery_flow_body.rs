/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.22
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateRecoveryFlowBody : Update Recovery Flow Request Body


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum UpdateRecoveryFlowBody {
    #[serde(rename="code")]
    UpdateRecoveryFlowWithCodeMethod {
        /// Code from recovery email  Sent to the user once a recovery has been initiated and is used to prove that the user is in possession of the email
        #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        code: Option<String>,
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Email to Recover  Needs to be set when initiating the flow. If the email is a registered recovery email, a recovery link will be sent. If the email is not known, a email with details on what happened will be sent instead.  format: email
        #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        email: Option<String>,
    },
    #[serde(rename="link")]
    UpdateRecoveryFlowWithLinkMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Email to Recover  Needs to be set when initiating the flow. If the email is a registered recovery email, a recovery link will be sent. If the email is not known, a email with details on what happened will be sent instead.  format: email
        #[serde(rename = "email")]
        // true, false, , String, false
        email: String,
    },
}





