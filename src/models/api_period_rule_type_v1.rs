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
pub struct ApiPeriodRuleTypeV1 {
    #[serde(rename = "channel")]
    pub channel: i32,
    #[serde(rename = "disposition_map")]
    pub disposition_map: Vec<models::DomainPeriodDisposition>,
    #[serde(rename = "fields")]
    pub fields: Vec<models::DomainPeriodField>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "long_desc")]
    pub long_desc: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "released")]
    pub released: bool,
}

impl ApiPeriodRuleTypeV1 {
    pub fn new(
        channel: i32,
        disposition_map: Vec<models::DomainPeriodDisposition>,
        fields: Vec<models::DomainPeriodField>,
        id: String,
        long_desc: String,
        name: String,
        platform: String,
        released: bool,
    ) -> ApiPeriodRuleTypeV1 {
        ApiPeriodRuleTypeV1 {
            channel,
            disposition_map,
            fields,
            id,
            long_desc,
            name,
            platform,
            released,
        }
    }
}
