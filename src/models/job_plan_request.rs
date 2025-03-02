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
pub struct JobPlanRequest {
    #[serde(rename = "Diff", skip_serializing_if = "Option::is_none")]
    pub diff: Option<bool>,
    #[serde(rename = "Job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::models::Job>>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "PolicyOverride", skip_serializing_if = "Option::is_none")]
    pub policy_override: Option<bool>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
}

impl JobPlanRequest {
    pub fn new() -> JobPlanRequest {
        JobPlanRequest {
            diff: None,
            job: None,
            namespace: None,
            policy_override: None,
            region: None,
            secret_id: None,
        }
    }
}


