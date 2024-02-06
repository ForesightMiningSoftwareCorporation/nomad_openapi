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
pub struct NodeDrainUpdateResponse {
    #[serde(rename = "EvalCreateIndex", skip_serializing_if = "Option::is_none")]
    pub eval_create_index: Option<i32>,
    #[serde(rename = "EvalIDs", skip_serializing_if = "Option::is_none")]
    pub eval_ids: Option<Vec<String>>,
    #[serde(rename = "LastIndex", skip_serializing_if = "Option::is_none")]
    pub last_index: Option<i32>,
    #[serde(rename = "NodeModifyIndex", skip_serializing_if = "Option::is_none")]
    pub node_modify_index: Option<i32>,
    #[serde(rename = "RequestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,
}

impl NodeDrainUpdateResponse {
    pub fn new() -> NodeDrainUpdateResponse {
        NodeDrainUpdateResponse {
            eval_create_index: None,
            eval_ids: None,
            last_index: None,
            node_modify_index: None,
            request_time: None,
        }
    }
}


