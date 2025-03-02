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
pub struct ServerHealth {
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastContact", skip_serializing_if = "Option::is_none")]
    pub last_contact: Option<i64>,
    #[serde(rename = "LastIndex", skip_serializing_if = "Option::is_none")]
    pub last_index: Option<i32>,
    #[serde(rename = "LastTerm", skip_serializing_if = "Option::is_none")]
    pub last_term: Option<i32>,
    #[serde(rename = "Leader", skip_serializing_if = "Option::is_none")]
    pub leader: Option<bool>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SerfStatus", skip_serializing_if = "Option::is_none")]
    pub serf_status: Option<String>,
    #[serde(rename = "StableSince", skip_serializing_if = "Option::is_none")]
    pub stable_since: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Voter", skip_serializing_if = "Option::is_none")]
    pub voter: Option<bool>,
}

impl ServerHealth {
    pub fn new() -> ServerHealth {
        ServerHealth {
            address: None,
            healthy: None,
            id: None,
            last_contact: None,
            last_index: None,
            last_term: None,
            leader: None,
            name: None,
            serf_status: None,
            stable_since: None,
            version: None,
            voter: None,
        }
    }
}


