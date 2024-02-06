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
pub struct VolumeMount {
    #[serde(rename = "Destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "PropagationMode", skip_serializing_if = "Option::is_none")]
    pub propagation_mode: Option<String>,
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
}

impl VolumeMount {
    pub fn new() -> VolumeMount {
        VolumeMount {
            destination: None,
            propagation_mode: None,
            read_only: None,
            volume: None,
        }
    }
}


