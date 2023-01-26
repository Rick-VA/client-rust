/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.7
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum UpdateLoginFlowBody {
    #[serde(rename="lookup_secret")]
    UpdateLoginFlowWithLookupSecretMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// The lookup secret.
        #[serde(rename = "lookup_secret")]
        // true, false, , String, false
        lookup_secret: String,
    },
    #[serde(rename="oidc")]
    UpdateLoginFlowWithOidcMethod {
        /// The CSRF Token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// The provider to register with
        #[serde(rename = "provider")]
        // true, false, , String, false
        provider: String,
        /// The identity traits. This is a placeholder for the registration flow.
        #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        traits: Option<serde_json::Value>,
    },
    #[serde(rename="password")]
    UpdateLoginFlowWithPasswordMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Identifier is the email or username of the user trying to log in.
        #[serde(rename = "identifier")]
        // true, false, , String, false
        identifier: String,
        /// The user's password.
        #[serde(rename = "password")]
        // true, false, , String, false
        password: String,
        /// Identifier is the email or username of the user trying to log in. This field is deprecated!
        #[serde(rename = "password_identifier", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        password_identifier: Option<String>,
    },
    #[serde(rename="totp")]
    UpdateLoginFlowWithTotpMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// The TOTP code.
        #[serde(rename = "totp_code")]
        // true, false, , String, false
        totp_code: String,
    },
    #[serde(rename="webauthn")]
    UpdateLoginFlowWithWebAuthnMethod {
        /// Sending the anti-csrf token is only required for browser login flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Identifier is the email or username of the user trying to log in.
        #[serde(rename = "identifier")]
        // true, false, , String, false
        identifier: String,
        /// Login a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
        #[serde(rename = "webauthn_login", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_login: Option<String>,
    },
}





