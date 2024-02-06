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
pub struct ChangeScript {
    #[serde(rename = "Args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "FailOnError", skip_serializing_if = "Option::is_none")]
    pub fail_on_error: Option<bool>,
    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl ChangeScript {
    pub fn new() -> ChangeScript {
        ChangeScript {
            args: None,
            command: None,
            fail_on_error: None,
            timeout: None,
        }
    }
}


