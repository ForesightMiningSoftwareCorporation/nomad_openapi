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
pub struct ConsulIngressConfigEntry {
    #[serde(rename = "Listeners", skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<crate::models::ConsulIngressListener>>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<crate::models::ConsulGatewayTlsConfig>>,
}

impl ConsulIngressConfigEntry {
    pub fn new() -> ConsulIngressConfigEntry {
        ConsulIngressConfigEntry {
            listeners: None,
            tls: None,
        }
    }
}


