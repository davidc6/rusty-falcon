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
pub struct DomainSensorInstallerV1 {
    /// installer description
    #[serde(rename = "description")]
    pub description: String,
    /// file size
    #[serde(rename = "file_size")]
    pub file_size: i32,
    /// file type
    #[serde(rename = "file_type")]
    pub file_type: String,
    /// installer file name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "os")]
    pub os: String,
    #[serde(rename = "os_version")]
    pub os_version: String,
    /// supported platform
    #[serde(rename = "platform")]
    pub platform: String,
    /// release date
    #[serde(rename = "release_date")]
    pub release_date: String,
    /// sha256
    #[serde(rename = "sha256")]
    pub sha256: String,
    /// version of the installer
    #[serde(rename = "version")]
    pub version: String,
}

impl DomainSensorInstallerV1 {
    pub fn new(description: String, file_size: i32, file_type: String, name: String, os: String, os_version: String, platform: String, release_date: String, sha256: String, version: String) -> DomainSensorInstallerV1 {
        DomainSensorInstallerV1 {
            description,
            file_size,
            file_type,
            name,
            os,
            os_version,
            platform,
            release_date,
            sha256,
            version,
        }
    }
}
