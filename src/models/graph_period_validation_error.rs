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
pub struct GraphPeriodValidationError {
    /// The cid the validation error applies to if it is not the CID managing the workflow
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    /// identifier for UI to indicate reason for node being invalid.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Display name for the property if it exists in a trigger or activity
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Level is used to indicate if this is an error or warning validation. If empty it should be assumed to be Error Level
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Detail of why the node is invalid.
    #[serde(rename = "message")]
    pub message: String,
    /// ID of the node in the graph that has been invalidated.
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// ID of the parent node in the graph that resulted in a child node being invalidated.
    #[serde(rename = "parent_node_id", skip_serializing_if = "Option::is_none")]
    pub parent_node_id: Option<String>,
    /// Used to specify an exact property that is invalid for fql evaluation
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    /// Resource ID for property if it exists, e.g. trigger ID or activity ID
    #[serde(rename = "resource_id", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl GraphPeriodValidationError {
    pub fn new(message: String) -> GraphPeriodValidationError {
        GraphPeriodValidationError {
            cid: None,
            code: None,
            display_name: None,
            level: None,
            message,
            node_id: None,
            parent_node_id: None,
            property: None,
            resource_id: None,
        }
    }
}
