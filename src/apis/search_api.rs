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


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_fuzzy_search`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFuzzySearchError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_search`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSearchError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}


#[allow(clippy::too_many_arguments)]
pub async fn get_fuzzy_search(configuration: &configuration::Configuration, fuzzy_search_request: crate::models::FuzzySearchRequest, region: Option<&str>, namespace: Option<&str>, index: Option<i32>, wait: Option<&str>, stale: Option<&str>, prefix: Option<&str>, x_nomad_token: Option<&str>, per_page: Option<i32>, next_token: Option<&str>) -> Result<crate::models::FuzzySearchResponse, Error<GetFuzzySearchError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/search/fuzzy", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = stale {
        local_var_req_builder = local_var_req_builder.query(&[("stale", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prefix {
        local_var_req_builder = local_var_req_builder.query(&[("prefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        local_var_req_builder = local_var_req_builder.query(&[("next_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = index {
        local_var_req_builder = local_var_req_builder.header("index", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&fuzzy_search_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFuzzySearchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn get_search(configuration: &configuration::Configuration, search_request: crate::models::SearchRequest, region: Option<&str>, namespace: Option<&str>, index: Option<i32>, wait: Option<&str>, stale: Option<&str>, prefix: Option<&str>, x_nomad_token: Option<&str>, per_page: Option<i32>, next_token: Option<&str>) -> Result<crate::models::SearchResponse, Error<GetSearchError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/search", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder = local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder = local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = stale {
        local_var_req_builder = local_var_req_builder.query(&[("stale", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prefix {
        local_var_req_builder = local_var_req_builder.query(&[("prefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        local_var_req_builder = local_var_req_builder.query(&[("next_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = index {
        local_var_req_builder = local_var_req_builder.header("index", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&search_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSearchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

