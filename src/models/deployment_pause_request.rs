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
pub struct DeploymentPauseRequest {
    #[serde(rename = "DeploymentID", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Pause", skip_serializing_if = "Option::is_none")]
    pub pause: Option<bool>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
}

impl DeploymentPauseRequest {
    pub fn new() -> DeploymentPauseRequest {
        DeploymentPauseRequest {
            deployment_id: None,
            namespace: None,
            pause: None,
            region: None,
            secret_id: None,
        }
    }
}


