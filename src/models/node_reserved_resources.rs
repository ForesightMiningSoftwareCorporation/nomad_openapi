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
pub struct NodeReservedResources {
    #[serde(rename = "Cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Box<crate::models::NodeReservedCpuResources>>,
    #[serde(rename = "Disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<Box<crate::models::NodeReservedDiskResources>>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<crate::models::NodeReservedMemoryResources>>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Box<crate::models::NodeReservedNetworkResources>>,
}

impl NodeReservedResources {
    pub fn new() -> NodeReservedResources {
        NodeReservedResources {
            cpu: None,
            disk: None,
            memory: None,
            networks: None,
        }
    }
}


