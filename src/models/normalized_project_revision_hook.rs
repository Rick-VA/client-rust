/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.16.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NormalizedProjectRevisionHook {
    /// The Hooks Config Key
    #[serde(rename = "config_key")]
    pub config_key: String,
    /// The Project's Revision Creation Date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The Hook Type
    #[serde(rename = "hook")]
    pub hook: String,
    /// ID of the entry
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Revision's ID this schema belongs to
    #[serde(rename = "project_revision_id", skip_serializing_if = "Option::is_none")]
    pub project_revision_id: Option<String>,
    /// Last Time Project's Revision was Updated
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Whether to send the API Key in the HTTP Header or as a HTTP Cookie
    #[serde(rename = "web_hook_config_auth_api_key_in", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_auth_api_key_in: Option<String>,
    /// The name of the api key
    #[serde(rename = "web_hook_config_auth_api_key_name", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_auth_api_key_name: Option<String>,
    /// The value of the api key
    #[serde(rename = "web_hook_config_auth_api_key_value", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_auth_api_key_value: Option<String>,
    /// The password to be sent in the HTTP Basic Auth Header
    #[serde(rename = "web_hook_config_auth_basic_auth_password", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_auth_basic_auth_password: Option<String>,
    /// The username to be sent in the HTTP Basic Auth Header
    #[serde(rename = "web_hook_config_auth_basic_auth_user", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_auth_basic_auth_user: Option<String>,
    /// HTTP Auth Method to use for the Web-Hook
    #[serde(rename = "web_hook_config_auth_type", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_auth_type: Option<String>,
    /// URI pointing to the JsonNet template used for Web-Hook payload generation. Only used for those HTTP methods, which support HTTP body payloads.
    #[serde(rename = "web_hook_config_body", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_body: Option<String>,
    /// If enabled allows the web hook to interrupt / abort the self-service flow. It only applies to certain flows (registration/verification/login/settings) and requires a valid response format.
    #[serde(rename = "web_hook_config_can_interrupt", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_can_interrupt: Option<bool>,
    /// The HTTP method to use (GET, POST, etc) for the Web-Hook
    #[serde(rename = "web_hook_config_method", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_method: Option<String>,
    /// Whether to ignore the Web Hook response
    #[serde(rename = "web_hook_config_response_ignore", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_response_ignore: Option<bool>,
    /// Whether to parse the Web Hook response
    #[serde(rename = "web_hook_config_response_parse", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_response_parse: Option<bool>,
    /// The URL the Web-Hook should call
    #[serde(rename = "web_hook_config_url", skip_serializing_if = "Option::is_none")]
    pub web_hook_config_url: Option<String>,
}

impl NormalizedProjectRevisionHook {
    pub fn new(config_key: String, hook: String) -> NormalizedProjectRevisionHook {
        NormalizedProjectRevisionHook {
            config_key,
            created_at: None,
            hook,
            id: None,
            project_revision_id: None,
            updated_at: None,
            web_hook_config_auth_api_key_in: None,
            web_hook_config_auth_api_key_name: None,
            web_hook_config_auth_api_key_value: None,
            web_hook_config_auth_basic_auth_password: None,
            web_hook_config_auth_basic_auth_user: None,
            web_hook_config_auth_type: None,
            web_hook_config_body: None,
            web_hook_config_can_interrupt: None,
            web_hook_config_method: None,
            web_hook_config_response_ignore: None,
            web_hook_config_response_parse: None,
            web_hook_config_url: None,
        }
    }
}

