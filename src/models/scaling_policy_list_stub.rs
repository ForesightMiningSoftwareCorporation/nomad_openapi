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
pub struct ScalingPolicyListStub {
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ScalingPolicyListStub {
    pub fn new() -> ScalingPolicyListStub {
        ScalingPolicyListStub {
            create_index: None,
            enabled: None,
            id: None,
            modify_index: None,
            target: None,
            _type: None,
        }
    }
}


