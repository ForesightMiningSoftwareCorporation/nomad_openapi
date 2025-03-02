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
pub struct TaskCsiPluginConfig {
    #[serde(rename = "HealthTimeout", skip_serializing_if = "Option::is_none")]
    pub health_timeout: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MountDir", skip_serializing_if = "Option::is_none")]
    pub mount_dir: Option<String>,
    #[serde(rename = "StagePublishBaseDir", skip_serializing_if = "Option::is_none")]
    pub stage_publish_base_dir: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl TaskCsiPluginConfig {
    pub fn new() -> TaskCsiPluginConfig {
        TaskCsiPluginConfig {
            health_timeout: None,
            id: None,
            mount_dir: None,
            stage_publish_base_dir: None,
            _type: None,
        }
    }
}


