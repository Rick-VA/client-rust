/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.17.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountExperienceConfiguration {
    #[serde(rename = "default_redirect_url")]
    pub default_redirect_url: String,
    #[serde(rename = "error_ui_url")]
    pub error_ui_url: String,
    #[serde(rename = "favicon_dark_url", skip_serializing_if = "Option::is_none")]
    pub favicon_dark_url: Option<String>,
    #[serde(rename = "favicon_light_url", skip_serializing_if = "Option::is_none")]
    pub favicon_light_url: Option<String>,
    #[serde(rename = "login_ui_url")]
    pub login_ui_url: String,
    #[serde(rename = "logo_dark_url", skip_serializing_if = "Option::is_none")]
    pub logo_dark_url: Option<String>,
    #[serde(rename = "logo_light_url", skip_serializing_if = "Option::is_none")]
    pub logo_light_url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "recovery_enabled")]
    pub recovery_enabled: bool,
    #[serde(rename = "recovery_ui_url")]
    pub recovery_ui_url: String,
    #[serde(rename = "registration_enabled")]
    pub registration_enabled: bool,
    #[serde(rename = "registration_ui_url")]
    pub registration_ui_url: String,
    #[serde(rename = "settings_ui_url")]
    pub settings_ui_url: String,
    #[serde(rename = "stylesheet", skip_serializing_if = "Option::is_none")]
    pub stylesheet: Option<String>,
    #[serde(rename = "verification_enabled")]
    pub verification_enabled: bool,
    #[serde(rename = "verification_ui_url")]
    pub verification_ui_url: String,
}

impl AccountExperienceConfiguration {
    pub fn new(default_redirect_url: String, error_ui_url: String, login_ui_url: String, name: String, recovery_enabled: bool, recovery_ui_url: String, registration_enabled: bool, registration_ui_url: String, settings_ui_url: String, verification_enabled: bool, verification_ui_url: String) -> AccountExperienceConfiguration {
        AccountExperienceConfiguration {
            default_redirect_url,
            error_ui_url,
            favicon_dark_url: None,
            favicon_light_url: None,
            login_ui_url,
            logo_dark_url: None,
            logo_light_url: None,
            name,
            recovery_enabled,
            recovery_ui_url,
            registration_enabled,
            registration_ui_url,
            settings_ui_url,
            stylesheet: None,
            verification_enabled,
            verification_ui_url,
        }
    }
}

