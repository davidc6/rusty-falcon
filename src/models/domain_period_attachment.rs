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
pub struct DomainPeriodAttachment {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "source_url", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}

impl DomainPeriodAttachment {
    pub fn new(link: String, mime_type: String) -> DomainPeriodAttachment {
        DomainPeriodAttachment {
            date: None,
            file_name: None,
            id: None,
            link,
            mime_type,
            sha256: None,
            source_url: None,
        }
    }
}
