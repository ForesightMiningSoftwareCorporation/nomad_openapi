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
pub struct SchedulerConfigurationResponse {
    #[serde(rename = "KnownLeader", skip_serializing_if = "Option::is_none")]
    pub known_leader: Option<bool>,
    #[serde(rename = "LastContact", skip_serializing_if = "Option::is_none")]
    pub last_contact: Option<i64>,
    #[serde(rename = "LastIndex", skip_serializing_if = "Option::is_none")]
    pub last_index: Option<i32>,
    #[serde(rename = "NextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,
    #[serde(rename = "SchedulerConfig", skip_serializing_if = "Option::is_none")]
    pub scheduler_config: Option<Box<crate::models::SchedulerConfiguration>>,
}

impl SchedulerConfigurationResponse {
    pub fn new() -> SchedulerConfigurationResponse {
        SchedulerConfigurationResponse {
            known_leader: None,
            last_contact: None,
            last_index: None,
            next_token: None,
            request_time: None,
            scheduler_config: None,
        }
    }
}


