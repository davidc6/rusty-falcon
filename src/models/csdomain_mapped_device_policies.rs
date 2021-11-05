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
pub struct CsdomainMappedDevicePolicies {
    #[serde(rename = "airlock", skip_serializing_if = "Option::is_none")]
    pub airlock: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "automox", skip_serializing_if = "Option::is_none")]
    pub automox: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "device_control", skip_serializing_if = "Option::is_none")]
    pub device_control: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "firewall", skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "global_config", skip_serializing_if = "Option::is_none")]
    pub global_config: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "netskope", skip_serializing_if = "Option::is_none")]
    pub netskope: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "prevention", skip_serializing_if = "Option::is_none")]
    pub prevention: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "remote_response", skip_serializing_if = "Option::is_none")]
    pub remote_response: Option<Box<crate::models::CsdomainDevicePolicy>>,
    #[serde(rename = "sensor_update", skip_serializing_if = "Option::is_none")]
    pub sensor_update: Option<Box<crate::models::CsdomainDevicePolicy>>,
}

impl CsdomainMappedDevicePolicies {
    pub fn new() -> CsdomainMappedDevicePolicies {
        CsdomainMappedDevicePolicies {
            airlock: None,
            automox: None,
            device_control: None,
            firewall: None,
            global_config: None,
            netskope: None,
            prevention: None,
            remote_response: None,
            sensor_update: None,
        }
    }
}
