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
pub struct Spread {
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "SpreadTarget", skip_serializing_if = "Option::is_none")]
    pub spread_target: Option<Vec<crate::models::SpreadTarget>>,
    #[serde(rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl Spread {
    pub fn new() -> Spread {
        Spread {
            attribute: None,
            spread_target: None,
            weight: None,
        }
    }
}


