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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountExperienceThemeVariables {
    #[serde(rename = "accent", skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(rename = "bg-default", skip_serializing_if = "Option::is_none")]
    pub bg_default: Option<String>,
    #[serde(rename = "border-radius-branding", skip_serializing_if = "Option::is_none")]
    pub border_radius_branding: Option<String>,
    #[serde(rename = "border-radius-buttons", skip_serializing_if = "Option::is_none")]
    pub border_radius_buttons: Option<String>,
    #[serde(rename = "border-radius-cards", skip_serializing_if = "Option::is_none")]
    pub border_radius_cards: Option<String>,
    #[serde(rename = "border-radius-forms", skip_serializing_if = "Option::is_none")]
    pub border_radius_forms: Option<String>,
    #[serde(rename = "border-radius-general", skip_serializing_if = "Option::is_none")]
    pub border_radius_general: Option<String>,
    #[serde(rename = "button-primary-bg-default", skip_serializing_if = "Option::is_none")]
    pub button_primary_bg_default: Option<String>,
    #[serde(rename = "button-primary-bg-disabled", skip_serializing_if = "Option::is_none")]
    pub button_primary_bg_disabled: Option<String>,
    #[serde(rename = "button-primary-bg-hover", skip_serializing_if = "Option::is_none")]
    pub button_primary_bg_hover: Option<String>,
    #[serde(rename = "button-primary-border-default", skip_serializing_if = "Option::is_none")]
    pub button_primary_border_default: Option<String>,
    #[serde(rename = "button-primary-border-disabled", skip_serializing_if = "Option::is_none")]
    pub button_primary_border_disabled: Option<String>,
    #[serde(rename = "button-primary-border-hover", skip_serializing_if = "Option::is_none")]
    pub button_primary_border_hover: Option<String>,
    #[serde(rename = "button-primary-fg-default", skip_serializing_if = "Option::is_none")]
    pub button_primary_fg_default: Option<String>,
    #[serde(rename = "button-primary-fg-disabled", skip_serializing_if = "Option::is_none")]
    pub button_primary_fg_disabled: Option<String>,
    #[serde(rename = "button-primary-fg-hover", skip_serializing_if = "Option::is_none")]
    pub button_primary_fg_hover: Option<String>,
    #[serde(rename = "button-secondary-bg-default", skip_serializing_if = "Option::is_none")]
    pub button_secondary_bg_default: Option<String>,
    #[serde(rename = "button-secondary-bg-disabled", skip_serializing_if = "Option::is_none")]
    pub button_secondary_bg_disabled: Option<String>,
    #[serde(rename = "button-secondary-bg-hover", skip_serializing_if = "Option::is_none")]
    pub button_secondary_bg_hover: Option<String>,
    #[serde(rename = "button-secondary-border-default", skip_serializing_if = "Option::is_none")]
    pub button_secondary_border_default: Option<String>,
    #[serde(rename = "button-secondary-border-disabled", skip_serializing_if = "Option::is_none")]
    pub button_secondary_border_disabled: Option<String>,
    #[serde(rename = "button-secondary-border-hover", skip_serializing_if = "Option::is_none")]
    pub button_secondary_border_hover: Option<String>,
    #[serde(rename = "button-secondary-fg-default", skip_serializing_if = "Option::is_none")]
    pub button_secondary_fg_default: Option<String>,
    #[serde(rename = "button-secondary-fg-disabled", skip_serializing_if = "Option::is_none")]
    pub button_secondary_fg_disabled: Option<String>,
    #[serde(rename = "button-secondary-fg-hover", skip_serializing_if = "Option::is_none")]
    pub button_secondary_fg_hover: Option<String>,
    #[serde(rename = "button-social-bg-default", skip_serializing_if = "Option::is_none")]
    pub button_social_bg_default: Option<String>,
    #[serde(rename = "button-social-bg-disabled", skip_serializing_if = "Option::is_none")]
    pub button_social_bg_disabled: Option<String>,
    #[serde(rename = "button-social-bg-hover", skip_serializing_if = "Option::is_none")]
    pub button_social_bg_hover: Option<String>,
    #[serde(rename = "button-social-bg-provider", skip_serializing_if = "Option::is_none")]
    pub button_social_bg_provider: Option<String>,
    #[serde(rename = "button-social-border-default", skip_serializing_if = "Option::is_none")]
    pub button_social_border_default: Option<String>,
    #[serde(rename = "button-social-border-disabled", skip_serializing_if = "Option::is_none")]
    pub button_social_border_disabled: Option<String>,
    #[serde(rename = "button-social-border-hover", skip_serializing_if = "Option::is_none")]
    pub button_social_border_hover: Option<String>,
    #[serde(rename = "button-social-border-provider", skip_serializing_if = "Option::is_none")]
    pub button_social_border_provider: Option<String>,
    #[serde(rename = "button-social-fg-default", skip_serializing_if = "Option::is_none")]
    pub button_social_fg_default: Option<String>,
    #[serde(rename = "button-social-fg-disabled", skip_serializing_if = "Option::is_none")]
    pub button_social_fg_disabled: Option<String>,
    #[serde(rename = "button-social-fg-hover", skip_serializing_if = "Option::is_none")]
    pub button_social_fg_hover: Option<String>,
    #[serde(rename = "button-social-fg-provider", skip_serializing_if = "Option::is_none")]
    pub button_social_fg_provider: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dialog-bg-default", skip_serializing_if = "Option::is_none")]
    pub dialog_bg_default: Option<String>,
    #[serde(rename = "dialog-bg-subtle", skip_serializing_if = "Option::is_none")]
    pub dialog_bg_subtle: Option<String>,
    #[serde(rename = "dialog-border-default", skip_serializing_if = "Option::is_none")]
    pub dialog_border_default: Option<String>,
    #[serde(rename = "dialog-fg-default", skip_serializing_if = "Option::is_none")]
    pub dialog_fg_default: Option<String>,
    #[serde(rename = "dialog-fg-mute", skip_serializing_if = "Option::is_none")]
    pub dialog_fg_mute: Option<String>,
    #[serde(rename = "dialog-fg-subtle", skip_serializing_if = "Option::is_none")]
    pub dialog_fg_subtle: Option<String>,
    #[serde(rename = "forms-bg-default", skip_serializing_if = "Option::is_none")]
    pub forms_bg_default: Option<String>,
    #[serde(rename = "forms-bg-disabled", skip_serializing_if = "Option::is_none")]
    pub forms_bg_disabled: Option<String>,
    #[serde(rename = "forms-bg-hover", skip_serializing_if = "Option::is_none")]
    pub forms_bg_hover: Option<String>,
    #[serde(rename = "forms-border-default", skip_serializing_if = "Option::is_none")]
    pub forms_border_default: Option<String>,
    #[serde(rename = "forms-border-disabled", skip_serializing_if = "Option::is_none")]
    pub forms_border_disabled: Option<String>,
    #[serde(rename = "forms-border-error", skip_serializing_if = "Option::is_none")]
    pub forms_border_error: Option<String>,
    #[serde(rename = "forms-border-focus", skip_serializing_if = "Option::is_none")]
    pub forms_border_focus: Option<String>,
    #[serde(rename = "forms-border-hover", skip_serializing_if = "Option::is_none")]
    pub forms_border_hover: Option<String>,
    #[serde(rename = "forms-border-success", skip_serializing_if = "Option::is_none")]
    pub forms_border_success: Option<String>,
    #[serde(rename = "forms-border-warn", skip_serializing_if = "Option::is_none")]
    pub forms_border_warn: Option<String>,
    #[serde(rename = "forms-checkbox-bg-checked", skip_serializing_if = "Option::is_none")]
    pub forms_checkbox_bg_checked: Option<String>,
    #[serde(rename = "forms-checkbox-bg-default", skip_serializing_if = "Option::is_none")]
    pub forms_checkbox_bg_default: Option<String>,
    #[serde(rename = "forms-checkbox-border-checked", skip_serializing_if = "Option::is_none")]
    pub forms_checkbox_border_checked: Option<String>,
    #[serde(rename = "forms-checkbox-border-default", skip_serializing_if = "Option::is_none")]
    pub forms_checkbox_border_default: Option<String>,
    #[serde(rename = "forms-checkbox-fg-checked", skip_serializing_if = "Option::is_none")]
    pub forms_checkbox_fg_checked: Option<String>,
    #[serde(rename = "forms-checkbox-fg-default", skip_serializing_if = "Option::is_none")]
    pub forms_checkbox_fg_default: Option<String>,
    #[serde(rename = "forms-fg-default", skip_serializing_if = "Option::is_none")]
    pub forms_fg_default: Option<String>,
    #[serde(rename = "forms-fg-error", skip_serializing_if = "Option::is_none")]
    pub forms_fg_error: Option<String>,
    #[serde(rename = "forms-fg-mute", skip_serializing_if = "Option::is_none")]
    pub forms_fg_mute: Option<String>,
    #[serde(rename = "forms-fg-subtle", skip_serializing_if = "Option::is_none")]
    pub forms_fg_subtle: Option<String>,
    #[serde(rename = "forms-fg-success", skip_serializing_if = "Option::is_none")]
    pub forms_fg_success: Option<String>,
    #[serde(rename = "forms-fg-warn", skip_serializing_if = "Option::is_none")]
    pub forms_fg_warn: Option<String>,
    #[serde(rename = "forms-radio-bg-checked", skip_serializing_if = "Option::is_none")]
    pub forms_radio_bg_checked: Option<String>,
    #[serde(rename = "forms-radio-bg-default", skip_serializing_if = "Option::is_none")]
    pub forms_radio_bg_default: Option<String>,
    #[serde(rename = "forms-radio-border-checked", skip_serializing_if = "Option::is_none")]
    pub forms_radio_border_checked: Option<String>,
    #[serde(rename = "forms-radio-border-default", skip_serializing_if = "Option::is_none")]
    pub forms_radio_border_default: Option<String>,
    #[serde(rename = "forms-radio-fg-checked", skip_serializing_if = "Option::is_none")]
    pub forms_radio_fg_checked: Option<String>,
    #[serde(rename = "forms-radio-fg-default", skip_serializing_if = "Option::is_none")]
    pub forms_radio_fg_default: Option<String>,
    #[serde(rename = "forms-toggle-bg-checked", skip_serializing_if = "Option::is_none")]
    pub forms_toggle_bg_checked: Option<String>,
    #[serde(rename = "forms-toggle-bg-default", skip_serializing_if = "Option::is_none")]
    pub forms_toggle_bg_default: Option<String>,
    #[serde(rename = "forms-toggle-border-checked", skip_serializing_if = "Option::is_none")]
    pub forms_toggle_border_checked: Option<String>,
    #[serde(rename = "forms-toggle-border-default", skip_serializing_if = "Option::is_none")]
    pub forms_toggle_border_default: Option<String>,
    #[serde(rename = "forms-toggle-fg-checked", skip_serializing_if = "Option::is_none")]
    pub forms_toggle_fg_checked: Option<String>,
    #[serde(rename = "forms-toggle-fg-default", skip_serializing_if = "Option::is_none")]
    pub forms_toggle_fg_default: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "links-link-default", skip_serializing_if = "Option::is_none")]
    pub links_link_default: Option<String>,
    #[serde(rename = "links-link-disabled", skip_serializing_if = "Option::is_none")]
    pub links_link_disabled: Option<String>,
    #[serde(rename = "links-link-hover", skip_serializing_if = "Option::is_none")]
    pub links_link_hover: Option<String>,
    #[serde(rename = "links-link-inverted-default", skip_serializing_if = "Option::is_none")]
    pub links_link_inverted_default: Option<String>,
    #[serde(rename = "links-link-inverted-hover", skip_serializing_if = "Option::is_none")]
    pub links_link_inverted_hover: Option<String>,
    #[serde(rename = "links-link-mute-default", skip_serializing_if = "Option::is_none")]
    pub links_link_mute_default: Option<String>,
    #[serde(rename = "links-link-mute-hover", skip_serializing_if = "Option::is_none")]
    pub links_link_mute_hover: Option<String>,
    #[serde(rename = "syntax-syntax", skip_serializing_if = "Option::is_none")]
    pub syntax_syntax: Option<String>,
    #[serde(rename = "syntax-syntax-key", skip_serializing_if = "Option::is_none")]
    pub syntax_syntax_key: Option<String>,
    #[serde(rename = "syntax-syntax-num", skip_serializing_if = "Option::is_none")]
    pub syntax_syntax_num: Option<String>,
    #[serde(rename = "syntax-syntax-value", skip_serializing_if = "Option::is_none")]
    pub syntax_syntax_value: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl AccountExperienceThemeVariables {
    pub fn new() -> AccountExperienceThemeVariables {
        AccountExperienceThemeVariables {
            accent: None,
            bg_default: None,
            border_radius_branding: None,
            border_radius_buttons: None,
            border_radius_cards: None,
            border_radius_forms: None,
            border_radius_general: None,
            button_primary_bg_default: None,
            button_primary_bg_disabled: None,
            button_primary_bg_hover: None,
            button_primary_border_default: None,
            button_primary_border_disabled: None,
            button_primary_border_hover: None,
            button_primary_fg_default: None,
            button_primary_fg_disabled: None,
            button_primary_fg_hover: None,
            button_secondary_bg_default: None,
            button_secondary_bg_disabled: None,
            button_secondary_bg_hover: None,
            button_secondary_border_default: None,
            button_secondary_border_disabled: None,
            button_secondary_border_hover: None,
            button_secondary_fg_default: None,
            button_secondary_fg_disabled: None,
            button_secondary_fg_hover: None,
            button_social_bg_default: None,
            button_social_bg_disabled: None,
            button_social_bg_hover: None,
            button_social_bg_provider: None,
            button_social_border_default: None,
            button_social_border_disabled: None,
            button_social_border_hover: None,
            button_social_border_provider: None,
            button_social_fg_default: None,
            button_social_fg_disabled: None,
            button_social_fg_hover: None,
            button_social_fg_provider: None,
            created_at: None,
            dialog_bg_default: None,
            dialog_bg_subtle: None,
            dialog_border_default: None,
            dialog_fg_default: None,
            dialog_fg_mute: None,
            dialog_fg_subtle: None,
            forms_bg_default: None,
            forms_bg_disabled: None,
            forms_bg_hover: None,
            forms_border_default: None,
            forms_border_disabled: None,
            forms_border_error: None,
            forms_border_focus: None,
            forms_border_hover: None,
            forms_border_success: None,
            forms_border_warn: None,
            forms_checkbox_bg_checked: None,
            forms_checkbox_bg_default: None,
            forms_checkbox_border_checked: None,
            forms_checkbox_border_default: None,
            forms_checkbox_fg_checked: None,
            forms_checkbox_fg_default: None,
            forms_fg_default: None,
            forms_fg_error: None,
            forms_fg_mute: None,
            forms_fg_subtle: None,
            forms_fg_success: None,
            forms_fg_warn: None,
            forms_radio_bg_checked: None,
            forms_radio_bg_default: None,
            forms_radio_border_checked: None,
            forms_radio_border_default: None,
            forms_radio_fg_checked: None,
            forms_radio_fg_default: None,
            forms_toggle_bg_checked: None,
            forms_toggle_bg_default: None,
            forms_toggle_border_checked: None,
            forms_toggle_border_default: None,
            forms_toggle_fg_checked: None,
            forms_toggle_fg_default: None,
            id: None,
            links_link_default: None,
            links_link_disabled: None,
            links_link_hover: None,
            links_link_inverted_default: None,
            links_link_inverted_hover: None,
            links_link_mute_default: None,
            links_link_mute_hover: None,
            syntax_syntax: None,
            syntax_syntax_key: None,
            syntax_syntax_num: None,
            syntax_syntax_value: None,
            updated_at: None,
        }
    }
}

