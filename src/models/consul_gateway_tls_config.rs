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
pub struct ConsulGatewayTlsConfig {
    #[serde(rename = "CipherSuites", skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<Vec<String>>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "TLSMaxVersion", skip_serializing_if = "Option::is_none")]
    pub tls_max_version: Option<String>,
    #[serde(rename = "TLSMinVersion", skip_serializing_if = "Option::is_none")]
    pub tls_min_version: Option<String>,
}

impl ConsulGatewayTlsConfig {
    pub fn new() -> ConsulGatewayTlsConfig {
        ConsulGatewayTlsConfig {
            cipher_suites: None,
            enabled: None,
            tls_max_version: None,
            tls_min_version: None,
        }
    }
}


