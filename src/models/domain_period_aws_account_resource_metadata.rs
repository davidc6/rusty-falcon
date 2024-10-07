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
pub struct DomainPeriodAwsAccountResourceMetadata {
    /// AWS CloudTrail bucket name to store logs.
    #[serde(
        rename = "aws_cloudtrail_bucket_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_cloudtrail_bucket_name: Option<String>,
    /// AWS CloudTrail region.
    #[serde(
        rename = "aws_cloudtrail_region",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_cloudtrail_region: Option<String>,
    /// AWS Eventbus ARN.
    #[serde(rename = "aws_eventbus_arn", skip_serializing_if = "Option::is_none")]
    pub aws_eventbus_arn: Option<String>,
    #[serde(rename = "eventbus_name", skip_serializing_if = "Option::is_none")]
    pub eventbus_name: Option<String>,
    /// ID assigned for use with cross account IAM role access.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The full arn of the IAM role created in this account to control access.
    #[serde(rename = "iam_role_arn", skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(
        rename = "intermediate_role_arn",
        skip_serializing_if = "Option::is_none"
    )]
    pub intermediate_role_arn: Option<String>,
}

impl DomainPeriodAwsAccountResourceMetadata {
    pub fn new() -> DomainPeriodAwsAccountResourceMetadata {
        DomainPeriodAwsAccountResourceMetadata {
            aws_cloudtrail_bucket_name: None,
            aws_cloudtrail_region: None,
            aws_eventbus_arn: None,
            eventbus_name: None,
            external_id: None,
            iam_role_arn: None,
            intermediate_role_arn: None,
        }
    }
}
