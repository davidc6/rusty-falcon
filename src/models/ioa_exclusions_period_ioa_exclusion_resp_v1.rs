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
pub struct IoaExclusionsPeriodIoaExclusionRespV1 {
    #[serde(rename = "applied_globally")]
    pub applied_globally: bool,
    #[serde(rename = "cl_regex")]
    pub cl_regex: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "detection_json")]
    pub detection_json: String,
    #[serde(rename = "groups")]
    pub groups: Vec<models::HostGroupsPeriodHostGroupV1>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ifn_regex")]
    pub ifn_regex: String,
    #[serde(rename = "last_modified")]
    pub last_modified: String,
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pattern_id")]
    pub pattern_id: String,
    #[serde(rename = "pattern_name")]
    pub pattern_name: String,
}

impl IoaExclusionsPeriodIoaExclusionRespV1 {
    pub fn new(
        applied_globally: bool,
        cl_regex: String,
        created_by: String,
        created_on: String,
        description: String,
        detection_json: String,
        groups: Vec<models::HostGroupsPeriodHostGroupV1>,
        id: String,
        ifn_regex: String,
        last_modified: String,
        modified_by: String,
        name: String,
        pattern_id: String,
        pattern_name: String,
    ) -> IoaExclusionsPeriodIoaExclusionRespV1 {
        IoaExclusionsPeriodIoaExclusionRespV1 {
            applied_globally,
            cl_regex,
            created_by,
            created_on,
            description,
            detection_json,
            groups,
            id,
            ifn_regex,
            last_modified,
            modified_by,
            name,
            pattern_id,
            pattern_name,
        }
    }
}
