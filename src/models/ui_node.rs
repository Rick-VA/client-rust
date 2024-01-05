/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.9
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UiNode : Nodes are represented as HTML elements or their native UI equivalents. For example, a node can be an `<img>` tag, or an `<input element>` but also `some plain text`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::models::UiNodeAttributes>,
    /// Group specifies which group (e.g. password authenticator) this node belongs to. default DefaultGroup password PasswordGroup oidc OpenIDConnectGroup profile ProfileGroup link LinkGroup code CodeGroup totp TOTPGroup lookup_secret LookupGroup webauthn WebAuthnGroup
    #[serde(rename = "group")]
    pub group: GroupEnum,
    #[serde(rename = "messages")]
    pub messages: Vec<crate::models::UiText>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::UiNodeMeta>,
    /// The node's type text Text input Input img Image a Anchor script Script
    #[serde(rename = "type")]
    pub _type: TypeEnum,
}


impl UiNode {
    /// Nodes are represented as HTML elements or their native UI equivalents. For example, a node can be an `<img>` tag, or an `<input element>` but also `some plain text`.
    pub fn new(attributes: crate::models::UiNodeAttributes, group: GroupEnum, messages: Vec<crate::models::UiText>, meta: crate::models::UiNodeMeta, _type: TypeEnum) -> UiNode {
        UiNode {
                attributes: Box::new(attributes),
                group,
                messages,
                meta: Box::new(meta),
                _type,
        }
    }
}

/// Group specifies which group (e.g. password authenticator) this node belongs to. default DefaultGroup password PasswordGroup oidc OpenIDConnectGroup profile ProfileGroup link LinkGroup code CodeGroup totp TOTPGroup lookup_secret LookupGroup webauthn WebAuthnGroup
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupEnum {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "oidc")]
    Oidc,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "lookup_secret")]
    LookupSecret,
    #[serde(rename = "webauthn")]
    Webauthn,
}
/// The node's type text Text input Input img Image a Anchor script Script
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "img")]
    Img,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "script")]
    Script,
}

