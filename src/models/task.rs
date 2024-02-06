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
pub struct Task {
    #[serde(rename = "Affinities", skip_serializing_if = "Option::is_none")]
    pub affinities: Option<Vec<crate::models::Affinity>>,
    #[serde(rename = "Artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<crate::models::TaskArtifact>>,
    #[serde(rename = "CSIPluginConfig", skip_serializing_if = "Option::is_none")]
    pub csi_plugin_config: Option<Box<crate::models::TaskCsiPluginConfig>>,
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "Constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<crate::models::Constraint>>,
    #[serde(rename = "DispatchPayload", skip_serializing_if = "Option::is_none")]
    pub dispatch_payload: Option<Box<crate::models::DispatchPayloadConfig>>,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "KillSignal", skip_serializing_if = "Option::is_none")]
    pub kill_signal: Option<String>,
    #[serde(rename = "KillTimeout", skip_serializing_if = "Option::is_none")]
    pub kill_timeout: Option<i64>,
    #[serde(rename = "Kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "Leader", skip_serializing_if = "Option::is_none")]
    pub leader: Option<bool>,
    #[serde(rename = "Lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<Box<crate::models::TaskLifecycle>>,
    #[serde(rename = "LogConfig", skip_serializing_if = "Option::is_none")]
    pub log_config: Option<Box<crate::models::LogConfig>>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<crate::models::Resources>>,
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<Box<crate::models::RestartPolicy>>,
    #[serde(rename = "ScalingPolicies", skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<crate::models::ScalingPolicy>>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::Service>>,
    #[serde(rename = "ShutdownDelay", skip_serializing_if = "Option::is_none")]
    pub shutdown_delay: Option<i64>,
    #[serde(rename = "Templates", skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<crate::models::Template>>,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "Vault", skip_serializing_if = "Option::is_none")]
    pub vault: Option<Box<crate::models::Vault>>,
    #[serde(rename = "VolumeMounts", skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<crate::models::VolumeMount>>,
}

impl Task {
    pub fn new() -> Task {
        Task {
            affinities: None,
            artifacts: None,
            csi_plugin_config: None,
            config: None,
            constraints: None,
            dispatch_payload: None,
            driver: None,
            env: None,
            kill_signal: None,
            kill_timeout: None,
            kind: None,
            leader: None,
            lifecycle: None,
            log_config: None,
            meta: None,
            name: None,
            resources: None,
            restart_policy: None,
            scaling_policies: None,
            services: None,
            shutdown_delay: None,
            templates: None,
            user: None,
            vault: None,
            volume_mounts: None,
        }
    }
}


