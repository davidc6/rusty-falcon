/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`get_mal_query_download_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMalQueryDownloadV1Error {
    Status400(models::MsaPeriodReplyMetaOnly),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status404(models::MsaPeriodErrorsOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mal_query_entities_samples_fetch_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMalQueryEntitiesSamplesFetchV1Error {
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MalqueryPeriodExternalQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mal_query_metadata_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMalQueryMetadataV1Error {
    Status400(models::MalqueryPeriodSampleMetadataResponse),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MalqueryPeriodSampleMetadataResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mal_query_quotas_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMalQueryQuotasV1Error {
    Status400(models::MsaPeriodErrorsOnly),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status404(models::MalqueryPeriodRateLimitsResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MalqueryPeriodRateLimitsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mal_query_request_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMalQueryRequestV1Error {
    Status400(models::MalqueryPeriodRequestResponse),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MalqueryPeriodRequestResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_mal_query_entities_samples_multidownload_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMalQueryEntitiesSamplesMultidownloadV1Error {
    Status400(models::MalqueryPeriodExternalQueryResponse),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status404(models::MalqueryPeriodExternalQueryResponse),
    Status429(models::MalqueryPeriodExternalQueryResponse),
    Status500(models::MalqueryPeriodExternalQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_mal_query_exact_search_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMalQueryExactSearchV1Error {
    Status400(models::MalqueryPeriodExternalQueryResponse),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status429(models::MalqueryPeriodExternalQueryResponse),
    Status500(models::MalqueryPeriodExternalQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_mal_query_fuzzy_search_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMalQueryFuzzySearchV1Error {
    Status400(models::MalqueryPeriodFuzzySearchResponse),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MalqueryPeriodFuzzySearchResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_mal_query_hunt_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMalQueryHuntV1Error {
    Status400(models::MalqueryPeriodExternalQueryResponse),
    Status401(models::MsaPeriodErrorsOnly),
    Status403(models::MsaPeriodErrorsOnly),
    Status429(models::MalqueryPeriodExternalQueryResponse),
    Status500(models::MalqueryPeriodExternalQueryResponse),
    UnknownValue(serde_json::Value),
}

pub async fn get_mal_query_download_v1(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<(), Error<GetMalQueryDownloadV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/entities/download-files/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(
            &ids.into_iter()
                .map(|p| ("ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => local_var_req_builder.query(&[(
            "ids",
            &ids.into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetMalQueryDownloadV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_mal_query_entities_samples_fetch_v1(
    configuration: &configuration::Configuration,
    ids: &str,
) -> Result<(), Error<GetMalQueryEntitiesSamplesFetchV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/entities/samples-fetch/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("ids", &ids.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetMalQueryEntitiesSamplesFetchV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_mal_query_metadata_v1(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::MalqueryPeriodSampleMetadataResponse, Error<GetMalQueryMetadataV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/entities/metadata/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(
            &ids.into_iter()
                .map(|p| ("ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => local_var_req_builder.query(&[(
            "ids",
            &ids.into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMalQueryMetadataV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_mal_query_quotas_v1(
    configuration: &configuration::Configuration,
) -> Result<models::MalqueryPeriodRateLimitsResponse, Error<GetMalQueryQuotasV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/aggregates/quotas/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMalQueryQuotasV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_mal_query_request_v1(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::MalqueryPeriodRequestResponse, Error<GetMalQueryRequestV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/entities/requests/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = match "csv" {
        "multi" => local_var_req_builder.query(
            &ids.into_iter()
                .map(|p| ("ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => local_var_req_builder.query(&[(
            "ids",
            &ids.into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMalQueryRequestV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_mal_query_entities_samples_multidownload_v1(
    configuration: &configuration::Configuration,
    body: models::MalqueryPeriodMultiDownloadRequestV1,
) -> Result<
    models::MalqueryPeriodExternalQueryResponse,
    Error<PostMalQueryEntitiesSamplesMultidownloadV1Error>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/entities/samples-multidownload/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostMalQueryEntitiesSamplesMultidownloadV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_mal_query_exact_search_v1(
    configuration: &configuration::Configuration,
    body: models::MalqueryPeriodExternalExactSearchParametersV1,
) -> Result<models::MalqueryPeriodExternalQueryResponse, Error<PostMalQueryExactSearchV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/queries/exact-search/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostMalQueryExactSearchV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_mal_query_fuzzy_search_v1(
    configuration: &configuration::Configuration,
    body: models::MalqueryPeriodFuzzySearchParametersV1,
) -> Result<models::MalqueryPeriodFuzzySearchResponse, Error<PostMalQueryFuzzySearchV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/combined/fuzzy-search/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostMalQueryFuzzySearchV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_mal_query_hunt_v1(
    configuration: &configuration::Configuration,
    body: models::MalqueryPeriodExternalHuntParametersV1,
) -> Result<models::MalqueryPeriodExternalQueryResponse, Error<PostMalQueryHuntV1Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/malquery/queries/hunt/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostMalQueryHuntV1Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
