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

/// InternalIsOwnerForProjectBySlugBody : Is Owner For Project By Slug Request Body
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalIsOwnerForProjectBySlugBody {
    /// Namespace is the namespace of the subject.
    #[serde(rename = "namespace")]
    pub namespace: NamespaceEnum,
    /// ProjectScope is the project_id resolved from the API key.
    #[serde(rename = "project_scope", skip_serializing_if = "Option::is_none")]
    pub project_scope: Option<String>,
    /// ProjectSlug is the project's slug.
    #[serde(rename = "project_slug")]
    pub project_slug: String,
    /// Subject is the subject acting (user or API key).
    #[serde(rename = "subject")]
    pub subject: String,
}

impl InternalIsOwnerForProjectBySlugBody {
    /// Is Owner For Project By Slug Request Body
    pub fn new(namespace: NamespaceEnum, project_slug: String, subject: String) -> InternalIsOwnerForProjectBySlugBody {
        InternalIsOwnerForProjectBySlugBody {
            namespace,
            project_scope: None,
            project_slug,
            subject,
        }
    }
}
/// Namespace is the namespace of the subject.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NamespaceEnum {
    #[serde(rename = "User")]
    User,
    #[serde(rename = " ApiKey")]
    ApiKey,
}

impl Default for NamespaceEnum {
    fn default() -> NamespaceEnum {
        Self::User
    }
}

