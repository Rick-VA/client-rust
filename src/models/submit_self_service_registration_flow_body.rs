/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.180
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum SubmitSelfServiceRegistrationFlowBody {
    #[serde(rename="oidc")]
    SubmitSelfServiceRegistrationFlowWithOidcMethodBody {
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        csrf_token: Option<String>,
        /// The provider to register with
        #[serde(rename = "provider")]
        provider: String,
        /// The identity traits
        #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
        traits: Option<serde_json::Value>,
    },
    #[serde(rename="password")]
    SubmitSelfServiceRegistrationFlowWithPasswordMethodBody {
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        csrf_token: Option<String>,
        /// Password to sign the user up with
        #[serde(rename = "password")]
        password: String,
        /// The identity's traits
        #[serde(rename = "traits")]
        traits: serde_json::Value,
    },
    #[serde(rename="webauthn")]
    SubmitSelfServiceRegistrationFlowWithWebAuthnMethodBody {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        csrf_token: Option<String>,
        /// The identity's traits
        #[serde(rename = "traits")]
        traits: serde_json::Value,
        /// Register a WebAuthn Security Key  It is expected that the JSON returned by the WebAuthn registration process is included here.
        #[serde(rename = "webauthn_register", skip_serializing_if = "Option::is_none")]
        webauthn_register: Option<String>,
        /// Name of the WebAuthn Security Key to be Added  A human-readable name for the security key which will be added.
        #[serde(rename = "webauthn_register_displayname", skip_serializing_if = "Option::is_none")]
        webauthn_register_displayname: Option<String>,
    },
}




