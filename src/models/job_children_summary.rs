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
pub struct JobChildrenSummary {
    #[serde(rename = "Dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<i64>,
    #[serde(rename = "Pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
}

impl JobChildrenSummary {
    pub fn new() -> JobChildrenSummary {
        JobChildrenSummary {
            dead: None,
            pending: None,
            running: None,
        }
    }
}


