/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct K8sregPeriodClusterResp {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "agent_version")]
    pub agent_version: Box<models::K8sregPeriodVersionResp>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
    #[serde(rename = "cluster_name")]
    pub cluster_name: String,
    #[serde(rename = "cluster_service")]
    pub cluster_service: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "helm_version")]
    pub helm_version: Box<models::K8sregPeriodVersionResp>,
    #[serde(rename = "k8s_version")]
    pub k8s_version: Box<models::K8sregPeriodVersionResp>,
    #[serde(rename = "last_heartbeat_at")]
    pub last_heartbeat_at: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl K8sregPeriodClusterResp {
    pub fn new(
        account_id: String,
        agent_version: models::K8sregPeriodVersionResp,
        cid: String,
        cluster_id: String,
        cluster_name: String,
        cluster_service: String,
        created_at: String,
        helm_version: models::K8sregPeriodVersionResp,
        k8s_version: models::K8sregPeriodVersionResp,
        last_heartbeat_at: String,
        location: String,
        status: String,
        updated_at: String,
    ) -> K8sregPeriodClusterResp {
        K8sregPeriodClusterResp {
            account_id,
            agent_version: Box::new(agent_version),
            cid,
            cluster_id,
            cluster_name,
            cluster_service,
            created_at,
            helm_version: Box::new(helm_version),
            k8s_version: Box::new(k8s_version),
            last_heartbeat_at,
            location,
            status,
            updated_at,
        }
    }
}
