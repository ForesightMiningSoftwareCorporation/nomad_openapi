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
pub struct JobRegisterRequest {
    #[serde(rename = "EnforceIndex", skip_serializing_if = "Option::is_none")]
    pub enforce_index: Option<bool>,
    #[serde(rename = "EvalPriority", skip_serializing_if = "Option::is_none")]
    pub eval_priority: Option<i32>,
    #[serde(rename = "Job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::models::Job>>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "PolicyOverride", skip_serializing_if = "Option::is_none")]
    pub policy_override: Option<bool>,
    #[serde(rename = "PreserveCounts", skip_serializing_if = "Option::is_none")]
    pub preserve_counts: Option<bool>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
}

impl JobRegisterRequest {
    pub fn new() -> JobRegisterRequest {
        JobRegisterRequest {
            enforce_index: None,
            eval_priority: None,
            job: None,
            job_modify_index: None,
            namespace: None,
            policy_override: None,
            preserve_counts: None,
            region: None,
            secret_id: None,
        }
    }
}


