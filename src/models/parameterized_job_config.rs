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
pub struct ParameterizedJobConfig {
    #[serde(rename = "MetaOptional", skip_serializing_if = "Option::is_none")]
    pub meta_optional: Option<Vec<String>>,
    #[serde(rename = "MetaRequired", skip_serializing_if = "Option::is_none")]
    pub meta_required: Option<Vec<String>>,
    #[serde(rename = "Payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl ParameterizedJobConfig {
    pub fn new() -> ParameterizedJobConfig {
        ParameterizedJobConfig {
            meta_optional: None,
            meta_required: None,
            payload: None,
        }
    }
}


