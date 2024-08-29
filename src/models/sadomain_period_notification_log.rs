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
pub struct SadomainPeriodNotificationLog {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "created_date")]
    pub created_date: String,
    #[serde(rename = "details")]
    pub details: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "notification_id")]
    pub notification_id: String,
    #[serde(rename = "user_email")]
    pub user_email: String,
    #[serde(rename = "user_uuid")]
    pub user_uuid: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl SadomainPeriodNotificationLog {
    pub fn new(
        action: String,
        cid: String,
        created_date: String,
        details: String,
        id: String,
        message: String,
        notification_id: String,
        user_email: String,
        user_uuid: String,
        username: String,
    ) -> SadomainPeriodNotificationLog {
        SadomainPeriodNotificationLog {
            action,
            cid,
            created_date,
            details,
            id,
            message,
            notification_id,
            user_email,
            user_uuid,
            username,
        }
    }
}
