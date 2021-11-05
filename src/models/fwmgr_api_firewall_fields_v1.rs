/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2021-10-05T19:33:53Z
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FwmgrApiFirewallFieldsV1 {
    #[serde(rename = "default_monitor")]
    pub default_monitor: Box<crate::models::FwmgrDomainMonitoring>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "platform_fields")]
    pub platform_fields: Vec<crate::models::FwmgrDomainField>,
}

impl FwmgrApiFirewallFieldsV1 {
    pub fn new(default_monitor: crate::models::FwmgrDomainMonitoring, id: String, platform: String, platform_fields: Vec<crate::models::FwmgrDomainField>) -> FwmgrApiFirewallFieldsV1 {
        FwmgrApiFirewallFieldsV1 {
            default_monitor: Box::new(default_monitor),
            id,
            platform,
            platform_fields,
        }
    }
}
