/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDeviceResource {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, crate::models::Attribute>>,
    #[serde(rename = "Instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::models::NodeDevice>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

impl NodeDeviceResource {
    pub fn new() -> NodeDeviceResource {
        NodeDeviceResource {
            attributes: None,
            instances: None,
            name: None,
            _type: None,
            vendor: None,
        }
    }
}


