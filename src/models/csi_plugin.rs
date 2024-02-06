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
pub struct CsiPlugin {
    #[serde(rename = "Allocations", skip_serializing_if = "Option::is_none")]
    pub allocations: Option<Vec<crate::models::AllocationListStub>>,
    #[serde(rename = "ControllerRequired", skip_serializing_if = "Option::is_none")]
    pub controller_required: Option<bool>,
    #[serde(rename = "Controllers", skip_serializing_if = "Option::is_none")]
    pub controllers: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "ControllersExpected", skip_serializing_if = "Option::is_none")]
    pub controllers_expected: Option<i32>,
    #[serde(rename = "ControllersHealthy", skip_serializing_if = "Option::is_none")]
    pub controllers_healthy: Option<i32>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "NodesExpected", skip_serializing_if = "Option::is_none")]
    pub nodes_expected: Option<i32>,
    #[serde(rename = "NodesHealthy", skip_serializing_if = "Option::is_none")]
    pub nodes_healthy: Option<i32>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl CsiPlugin {
    pub fn new() -> CsiPlugin {
        CsiPlugin {
            allocations: None,
            controller_required: None,
            controllers: None,
            controllers_expected: None,
            controllers_healthy: None,
            create_index: None,
            id: None,
            modify_index: None,
            nodes: None,
            nodes_expected: None,
            nodes_healthy: None,
            provider: None,
            version: None,
        }
    }
}


