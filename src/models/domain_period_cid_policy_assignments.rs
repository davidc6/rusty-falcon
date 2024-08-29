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
pub struct DomainPeriodCidPolicyAssignments {
    #[serde(rename = "account_scope", skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "attack_types", skip_serializing_if = "Option::is_none")]
    pub attack_types: Option<Vec<String>>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cis_benchmark", skip_serializing_if = "Option::is_none")]
    pub cis_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "cisa_benchmark", skip_serializing_if = "Option::is_none")]
    pub cisa_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "cloud_asset_type", skip_serializing_if = "Option::is_none")]
    pub cloud_asset_type: Option<String>,
    #[serde(
        rename = "cloud_asset_type_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_asset_type_id: Option<i32>,
    #[serde(rename = "cloud_provider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    #[serde(rename = "cloud_service", skip_serializing_if = "Option::is_none")]
    pub cloud_service: Option<String>,
    #[serde(
        rename = "cloud_service_friendly",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_service_friendly: Option<String>,
    #[serde(
        rename = "cloud_service_subtype",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_service_subtype: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "default_severity", skip_serializing_if = "Option::is_none")]
    pub default_severity: Option<String>,
    #[serde(rename = "fql_policy", skip_serializing_if = "Option::is_none")]
    pub fql_policy: Option<String>,
    #[serde(rename = "hipaa_benchmark", skip_serializing_if = "Option::is_none")]
    pub hipaa_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "hitrust_benchmark", skip_serializing_if = "Option::is_none")]
    pub hitrust_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "is_remediable")]
    pub is_remediable: bool,
    #[serde(rename = "iso_benchmark", skip_serializing_if = "Option::is_none")]
    pub iso_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nist_benchmark", skip_serializing_if = "Option::is_none")]
    pub nist_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "pci_benchmark", skip_serializing_if = "Option::is_none")]
    pub pci_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<i32>,
    #[serde(rename = "policy_settings", skip_serializing_if = "Option::is_none")]
    pub policy_settings: Option<Vec<models::DomainPeriodPolicySettingByAccountAndRegion>>,
    #[serde(rename = "policy_timestamp", skip_serializing_if = "Option::is_none")]
    pub policy_timestamp: Option<String>,
    #[serde(rename = "policy_type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(
        rename = "remediation_summary",
        skip_serializing_if = "Option::is_none"
    )]
    pub remediation_summary: Option<String>,
    #[serde(rename = "soc2_benchmark", skip_serializing_if = "Option::is_none")]
    pub soc2_benchmark: Option<Vec<models::DomainPeriodBenchmark>>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl DomainPeriodCidPolicyAssignments {
    pub fn new(is_remediable: bool) -> DomainPeriodCidPolicyAssignments {
        DomainPeriodCidPolicyAssignments {
            account_scope: None,
            attack_types: None,
            cid: None,
            cis_benchmark: None,
            cisa_benchmark: None,
            cloud_asset_type: None,
            cloud_asset_type_id: None,
            cloud_provider: None,
            cloud_service: None,
            cloud_service_friendly: None,
            cloud_service_subtype: None,
            created_at: None,
            default_severity: None,
            fql_policy: None,
            hipaa_benchmark: None,
            hitrust_benchmark: None,
            is_remediable,
            iso_benchmark: None,
            name: None,
            nist_benchmark: None,
            pci_benchmark: None,
            policy_id: None,
            policy_settings: None,
            policy_timestamp: None,
            policy_type: None,
            remediation_summary: None,
            soc2_benchmark: None,
            updated_at: None,
        }
    }
}
