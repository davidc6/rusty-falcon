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
pub struct DomainPeriodPublicIndicatorV3 {
    #[serde(rename = "_marker")]
    pub _marker: String,
    #[serde(rename = "actors")]
    pub actors: Vec<String>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "domain_types")]
    pub domain_types: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "indicator")]
    pub indicator: String,
    #[serde(rename = "ip_address_types")]
    pub ip_address_types: Vec<String>,
    #[serde(rename = "kill_chains")]
    pub kill_chains: Vec<String>,
    #[serde(rename = "labels")]
    pub labels: Vec<models::DomainPeriodCsixLabel>,
    #[serde(rename = "last_updated")]
    pub last_updated: i64,
    #[serde(rename = "malicious_confidence")]
    pub malicious_confidence: String,
    #[serde(rename = "malware_families")]
    pub malware_families: Vec<String>,
    #[serde(rename = "published_date")]
    pub published_date: i64,
    #[serde(rename = "relations")]
    pub relations: Vec<models::DomainPeriodCsixRelation>,
    #[serde(rename = "reports")]
    pub reports: Vec<String>,
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
    #[serde(rename = "threat_types")]
    pub threat_types: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "vulnerabilities")]
    pub vulnerabilities: Vec<String>,
}

impl DomainPeriodPublicIndicatorV3 {
    pub fn new(
        _marker: String,
        actors: Vec<String>,
        deleted: bool,
        domain_types: Vec<String>,
        id: String,
        indicator: String,
        ip_address_types: Vec<String>,
        kill_chains: Vec<String>,
        labels: Vec<models::DomainPeriodCsixLabel>,
        last_updated: i64,
        malicious_confidence: String,
        malware_families: Vec<String>,
        published_date: i64,
        relations: Vec<models::DomainPeriodCsixRelation>,
        reports: Vec<String>,
        targets: Vec<String>,
        threat_types: Vec<String>,
        r#type: String,
        vulnerabilities: Vec<String>,
    ) -> DomainPeriodPublicIndicatorV3 {
        DomainPeriodPublicIndicatorV3 {
            _marker,
            actors,
            deleted,
            domain_types,
            id,
            indicator,
            ip_address_types,
            kill_chains,
            labels,
            last_updated,
            malicious_confidence,
            malware_families,
            published_date,
            relations,
            reports,
            targets,
            threat_types,
            r#type,
            vulnerabilities,
        }
    }
}
