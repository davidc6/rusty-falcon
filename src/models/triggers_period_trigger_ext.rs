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
pub struct TriggersPeriodTriggerExt {
    /// Required api scope to use this trigger.
    #[serde(rename = "api_scope")]
    pub api_scope: String,
    /// Category for the trigger.
    #[serde(rename = "category")]
    pub category: String,
    /// Description for the trigger.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nested fields of this object, only set when field type is complex
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<models::TriggersPeriodTriggerExtField>>,
    /// Unique identifier for the trigger.
    #[serde(rename = "id")]
    pub id: String,
    /// User friendly display name for the trigger, this the fully qualified name that provides the context hierarchy.
    #[serde(rename = "name")]
    pub name: String,
    /// Version of the trigger.
    #[serde(rename = "version")]
    pub version: String,
}

impl TriggersPeriodTriggerExt {
    pub fn new(
        api_scope: String,
        category: String,
        id: String,
        name: String,
        version: String,
    ) -> TriggersPeriodTriggerExt {
        TriggersPeriodTriggerExt {
            api_scope,
            category,
            description: None,
            fields: None,
            id,
            name,
            version,
        }
    }
}
