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
pub struct ConsulProxy {
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "ExposeConfig", skip_serializing_if = "Option::is_none")]
    pub expose_config: Option<Box<crate::models::ConsulExposeConfig>>,
    #[serde(rename = "LocalServiceAddress", skip_serializing_if = "Option::is_none")]
    pub local_service_address: Option<String>,
    #[serde(rename = "LocalServicePort", skip_serializing_if = "Option::is_none")]
    pub local_service_port: Option<i32>,
    #[serde(rename = "Upstreams", skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<crate::models::ConsulUpstream>>,
}

impl ConsulProxy {
    pub fn new() -> ConsulProxy {
        ConsulProxy {
            config: None,
            expose_config: None,
            local_service_address: None,
            local_service_port: None,
            upstreams: None,
        }
    }
}


