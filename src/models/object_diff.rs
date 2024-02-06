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
pub struct ObjectDiff {
    #[serde(rename = "Fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::FieldDiff>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Objects", skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<crate::models::ObjectDiff>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ObjectDiff {
    pub fn new() -> ObjectDiff {
        ObjectDiff {
            fields: None,
            name: None,
            objects: None,
            _type: None,
        }
    }
}


