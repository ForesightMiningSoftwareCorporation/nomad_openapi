// Copyright (c) HashiCorp, Inc.
// SPDX-License-Identifier: MPL-2.0

/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceCapabilities {
    #[serde(rename = "DisabledTaskDrivers", skip_serializing_if = "Option::is_none")]
    pub disabled_task_drivers: Option<Vec<String>>,
    #[serde(rename = "EnabledTaskDrivers", skip_serializing_if = "Option::is_none")]
    pub enabled_task_drivers: Option<Vec<String>>,
}

impl NamespaceCapabilities {
    pub fn new() -> NamespaceCapabilities {
        NamespaceCapabilities {
            disabled_task_drivers: None,
            enabled_task_drivers: None,
        }
    }
}


