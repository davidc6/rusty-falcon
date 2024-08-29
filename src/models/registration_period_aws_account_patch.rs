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
pub struct RegistrationPeriodAwsAccountPatch {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(
        rename = "behavior_assessment_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_assessment_enabled: Option<bool>,
    #[serde(rename = "cloudtrail_region", skip_serializing_if = "Option::is_none")]
    pub cloudtrail_region: Option<String>,
    #[serde(rename = "dspm_enabled", skip_serializing_if = "Option::is_none")]
    pub dspm_enabled: Option<bool>,
    #[serde(rename = "dspm_role", skip_serializing_if = "Option::is_none")]
    pub dspm_role: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "iam_role_arn")]
    pub iam_role_arn: String,
    #[serde(rename = "remediation_region", skip_serializing_if = "Option::is_none")]
    pub remediation_region: Option<String>,
    #[serde(
        rename = "remediation_tou_accepted",
        skip_serializing_if = "Option::is_none"
    )]
    pub remediation_tou_accepted: Option<String>,
    #[serde(
        rename = "sensor_management_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_management_enabled: Option<bool>,
    #[serde(rename = "target_ous", skip_serializing_if = "Option::is_none")]
    pub target_ous: Option<Vec<String>>,
}

impl RegistrationPeriodAwsAccountPatch {
    pub fn new(account_id: String, iam_role_arn: String) -> RegistrationPeriodAwsAccountPatch {
        RegistrationPeriodAwsAccountPatch {
            account_id,
            behavior_assessment_enabled: None,
            cloudtrail_region: None,
            dspm_enabled: None,
            dspm_role: None,
            environment: None,
            iam_role_arn,
            remediation_region: None,
            remediation_tou_accepted: None,
            sensor_management_enabled: None,
            target_ous: None,
        }
    }
}
