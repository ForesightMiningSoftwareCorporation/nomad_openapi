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
pub struct TaskLifecycle {
    #[serde(rename = "Hook", skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[serde(rename = "Sidecar", skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<bool>,
}

impl TaskLifecycle {
    pub fn new() -> TaskLifecycle {
        TaskLifecycle {
            hook: None,
            sidecar: None,
        }
    }
}


