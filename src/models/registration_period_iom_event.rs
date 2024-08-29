/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationPeriodIomEvent {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "account_name")]
    pub account_name: String,
    #[serde(rename = "azure_tenant_id")]
    pub azure_tenant_id: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: String,
    #[serde(rename = "custom_policy_id", skip_serializing_if = "Option::is_none")]
    pub custom_policy_id: Option<String>,
    #[serde(rename = "finding")]
    pub finding: String,
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "policy_statement")]
    pub policy_statement: String,
    #[serde(rename = "policy_type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "report_date_time")]
    pub report_date_time: String,
    #[serde(rename = "resource_attributes")]
    pub resource_attributes: String,
    #[serde(rename = "resource_create_time")]
    pub resource_create_time: String,
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    #[serde(rename = "resource_id_type")]
    pub resource_id_type: String,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tags")]
    pub tags: String,
}

impl RegistrationPeriodIomEvent {
    pub fn new(
        account_id: String,
        account_name: String,
        azure_tenant_id: String,
        cid: String,
        cloud_provider: String,
        finding: String,
        policy_statement: String,
        region: String,
        report_date_time: String,
        resource_attributes: String,
        resource_create_time: String,
        resource_id: String,
        resource_id_type: String,
        resource_url: String,
        service: String,
        severity: String,
        status: String,
        tags: String,
    ) -> RegistrationPeriodIomEvent {
        RegistrationPeriodIomEvent {
            account_id,
            account_name,
            azure_tenant_id,
            cid,
            cloud_provider,
            custom_policy_id: None,
            finding,
            policy_id: None,
            policy_statement,
            policy_type: None,
            region,
            report_date_time,
            resource_attributes,
            resource_create_time,
            resource_id,
            resource_id_type,
            resource_url,
            service,
            severity,
            status,
            tags,
        }
    }
}
