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
pub struct ConsulSidecarService {
    #[serde(rename = "DisableDefaultTCPCheck", skip_serializing_if = "Option::is_none")]
    pub disable_default_tcp_check: Option<bool>,
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "Proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Box<crate::models::ConsulProxy>>,
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl ConsulSidecarService {
    pub fn new() -> ConsulSidecarService {
        ConsulSidecarService {
            disable_default_tcp_check: None,
            port: None,
            proxy: None,
            tags: None,
        }
    }
}


