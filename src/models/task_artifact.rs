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
pub struct TaskArtifact {
    #[serde(rename = "GetterHeaders", skip_serializing_if = "Option::is_none")]
    pub getter_headers: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "GetterMode", skip_serializing_if = "Option::is_none")]
    pub getter_mode: Option<String>,
    #[serde(rename = "GetterOptions", skip_serializing_if = "Option::is_none")]
    pub getter_options: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "GetterSource", skip_serializing_if = "Option::is_none")]
    pub getter_source: Option<String>,
    #[serde(rename = "RelativeDest", skip_serializing_if = "Option::is_none")]
    pub relative_dest: Option<String>,
}

impl TaskArtifact {
    pub fn new() -> TaskArtifact {
        TaskArtifact {
            getter_headers: None,
            getter_mode: None,
            getter_options: None,
            getter_source: None,
            relative_dest: None,
        }
    }
}


