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
pub struct DeploymentAllocHealthRequest {
    #[serde(rename = "DeploymentID", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "HealthyAllocationIDs", skip_serializing_if = "Option::is_none")]
    pub healthy_allocation_ids: Option<Vec<String>>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
    #[serde(rename = "UnhealthyAllocationIDs", skip_serializing_if = "Option::is_none")]
    pub unhealthy_allocation_ids: Option<Vec<String>>,
}

impl DeploymentAllocHealthRequest {
    pub fn new() -> DeploymentAllocHealthRequest {
        DeploymentAllocHealthRequest {
            deployment_id: None,
            healthy_allocation_ids: None,
            namespace: None,
            region: None,
            secret_id: None,
            unhealthy_allocation_ids: None,
        }
    }
}


