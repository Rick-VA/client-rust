/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.9
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// Identity : An [identity](https://www.ory.sh/docs/kratos/concepts/identity-user-model) represents a (human) user in Ory.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    /// CreatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Credentials represents all credentials that can be used for authenticating this identity.
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<::std::collections::HashMap<String, crate::models::IdentityCredentials>>,
    /// ID is the identity's unique identifier.  The Identity ID can not be changed and can not be chosen. This ensures future compatibility and optimization for distributed stores such as CockroachDB.
    #[serde(rename = "id")]
    pub id: String,
    /// NullJSONRawMessage represents a json.RawMessage that works well with JSON, SQL, and Swagger and is NULLable-
    #[serde(rename = "metadata_admin", skip_serializing_if = "Option::is_none")]
    pub metadata_admin: Option<serde_json::Value>,
    /// NullJSONRawMessage represents a json.RawMessage that works well with JSON, SQL, and Swagger and is NULLable-
    #[serde(rename = "metadata_public", skip_serializing_if = "Option::is_none")]
    pub metadata_public: Option<serde_json::Value>,
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// RecoveryAddresses contains all the addresses that can be used to recover an identity.
    #[serde(rename = "recovery_addresses", skip_serializing_if = "Option::is_none")]
    pub recovery_addresses: Option<Vec<crate::models::RecoveryIdentityAddress>>,
    /// SchemaID is the ID of the JSON Schema to be used for validating the identity's traits.
    #[serde(rename = "schema_id")]
    pub schema_id: String,
    /// SchemaURL is the URL of the endpoint where the identity's traits schema can be fetched from.  format: url
    #[serde(rename = "schema_url")]
    pub schema_url: String,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::IdentityState>,
    #[serde(rename = "state_changed_at", skip_serializing_if = "Option::is_none")]
    pub state_changed_at: Option<String>,
    /// Traits represent an identity's traits. The identity is able to create, modify, and delete traits in a self-service manner. The input will always be validated against the JSON Schema defined in `schema_url`.
    #[serde(rename = "traits")]
    pub traits: Option<serde_json::Value>,
    /// UpdatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// VerifiableAddresses contains all the addresses that can be verified by the user.
    #[serde(rename = "verifiable_addresses", skip_serializing_if = "Option::is_none")]
    pub verifiable_addresses: Option<Vec<crate::models::VerifiableIdentityAddress>>,
}


impl Identity {
    /// An [identity](https://www.ory.sh/docs/kratos/concepts/identity-user-model) represents a (human) user in Ory.
    pub fn new(id: String, schema_id: String, schema_url: String, traits: Option<serde_json::Value>) -> Identity {
        Identity {
                created_at: None,
                credentials: None,
                id,
                metadata_admin: None,
                metadata_public: None,
                organization_id: None,
                recovery_addresses: None,
                schema_id,
                schema_url,
                state: None,
                state_changed_at: None,
                traits,
                updated_at: None,
                verifiable_addresses: None,
        }
    }
}


