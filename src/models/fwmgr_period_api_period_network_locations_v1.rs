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
pub struct FwmgrPeriodApiPeriodNetworkLocationsV1 {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "connection_types")]
    pub connection_types: Box<models::FwmgrPeriodDomainPeriodConnectionType>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "created_on", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "default_gateways")]
    pub default_gateways: Vec<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "dhcp_servers")]
    pub dhcp_servers: Vec<String>,
    #[serde(rename = "dns_resolution_targets")]
    pub dns_resolution_targets: Box<models::FwmgrPeriodDomainPeriodDnsResolutionTargetsWithPolling>,
    #[serde(rename = "dns_servers")]
    pub dns_servers: Vec<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "host_addresses")]
    pub host_addresses: Vec<String>,
    #[serde(rename = "https_reachable_hosts")]
    pub https_reachable_hosts: Box<models::FwmgrPeriodDomainPeriodHttpsHostsWithPolling>,
    #[serde(rename = "icmp_request_targets")]
    pub icmp_request_targets: Box<models::FwmgrPeriodDomainPeriodIcmpTargetsWithPolling>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::FwmgrPeriodApiPeriodNetworkLocationsMetadataV1>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "modified_on", skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rule_count")]
    pub rule_count: i32,
}

impl FwmgrPeriodApiPeriodNetworkLocationsV1 {
    pub fn new(
        cid: String,
        connection_types: models::FwmgrPeriodDomainPeriodConnectionType,
        default_gateways: Vec<String>,
        description: String,
        dhcp_servers: Vec<String>,
        dns_resolution_targets: models::FwmgrPeriodDomainPeriodDnsResolutionTargetsWithPolling,
        dns_servers: Vec<String>,
        enabled: bool,
        host_addresses: Vec<String>,
        https_reachable_hosts: models::FwmgrPeriodDomainPeriodHttpsHostsWithPolling,
        icmp_request_targets: models::FwmgrPeriodDomainPeriodIcmpTargetsWithPolling,
        id: String,
        name: String,
        rule_count: i32,
    ) -> FwmgrPeriodApiPeriodNetworkLocationsV1 {
        FwmgrPeriodApiPeriodNetworkLocationsV1 {
            cid,
            connection_types: Box::new(connection_types),
            created_by: None,
            created_on: None,
            default_gateways,
            description,
            dhcp_servers,
            dns_resolution_targets: Box::new(dns_resolution_targets),
            dns_servers,
            enabled,
            host_addresses,
            https_reachable_hosts: Box::new(https_reachable_hosts),
            icmp_request_targets: Box::new(icmp_request_targets),
            id,
            metadata: None,
            modified_by: None,
            modified_on: None,
            name,
            rule_count,
        }
    }
}
