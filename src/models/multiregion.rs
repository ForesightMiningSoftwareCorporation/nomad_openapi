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
pub struct Multiregion {
    #[serde(rename = "Regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<crate::models::MultiregionRegion>>,
    #[serde(rename = "Strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Box<crate::models::MultiregionStrategy>>,
}

impl Multiregion {
    pub fn new() -> Multiregion {
        Multiregion {
            regions: None,
            strategy: None,
        }
    }
}


