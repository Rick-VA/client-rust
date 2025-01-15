/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.16.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateLoginFlowWithCodeMethod : Update Login flow using the code method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLoginFlowWithCodeMethod {
    /// Address is the address to send the code to, in case that there are multiple addresses. This field is only used in two-factor flows and is ineffective for passwordless flows.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Code is the 6 digits code sent to the user
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token")]
    pub csrf_token: String,
    /// Identifier is the code identifier The identifier requires that the user has already completed the registration or settings with code flow.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Method should be set to \"code\" when logging in using the code strategy.
    #[serde(rename = "method")]
    pub method: String,
    /// Resend is set when the user wants to resend the code
    #[serde(rename = "resend", skip_serializing_if = "Option::is_none")]
    pub resend: Option<String>,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateLoginFlowWithCodeMethod {
    /// Update Login flow using the code method
    pub fn new(csrf_token: String, method: String) -> UpdateLoginFlowWithCodeMethod {
        UpdateLoginFlowWithCodeMethod {
            address: None,
            code: None,
            csrf_token,
            identifier: None,
            method,
            resend: None,
            transient_payload: None,
        }
    }
}

