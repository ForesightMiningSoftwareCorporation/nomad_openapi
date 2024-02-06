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
pub struct WaitConfig {
    #[serde(rename = "Max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
}

impl WaitConfig {
    pub fn new() -> WaitConfig {
        WaitConfig {
            max: None,
            min: None,
        }
    }
}


