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
pub struct NodeUpdateDrainRequest {
    #[serde(rename = "DrainSpec", skip_serializing_if = "Option::is_none")]
    pub drain_spec: Option<Box<crate::models::DrainSpec>>,
    #[serde(rename = "MarkEligible", skip_serializing_if = "Option::is_none")]
    pub mark_eligible: Option<bool>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl NodeUpdateDrainRequest {
    pub fn new() -> NodeUpdateDrainRequest {
        NodeUpdateDrainRequest {
            drain_spec: None,
            mark_eligible: None,
            meta: None,
            node_id: None,
        }
    }
}


