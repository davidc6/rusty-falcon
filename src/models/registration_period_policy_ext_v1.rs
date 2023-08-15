/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2023-08-08T23:00:01Z
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RegistrationPeriodPolicyExtV1 {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "account_ids")]
    pub account_ids: Vec<String>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "policy_id")]
    pub policy_id: i32,
    #[serde(rename = "regions")]
    pub regions: Vec<String>,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "tag_excluded")]
    pub tag_excluded: bool,
}

impl RegistrationPeriodPolicyExtV1 {
    pub fn new(
        account_id: String,
        account_ids: Vec<String>,
        enabled: bool,
        policy_id: i32,
        regions: Vec<String>,
        severity: String,
        tag_excluded: bool,
    ) -> RegistrationPeriodPolicyExtV1 {
        RegistrationPeriodPolicyExtV1 {
            account_id,
            account_ids,
            enabled,
            policy_id,
            regions,
            severity,
            tag_excluded,
        }
    }
}
