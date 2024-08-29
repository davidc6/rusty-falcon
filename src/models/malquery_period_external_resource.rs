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
pub struct MalqueryPeriodExternalResource {
    /// Sample family
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// Sample size
    #[serde(rename = "filesize", skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i32>,
    /// Sample file type
    #[serde(rename = "filetype", skip_serializing_if = "Option::is_none")]
    pub filetype: Option<String>,
    /// Date when it was first seen
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    /// Reason why the resource is ignored
    #[serde(rename = "ignore_reason", skip_serializing_if = "Option::is_none")]
    pub ignore_reason: Option<String>,
    /// Sample label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Resource label confidence
    #[serde(rename = "label_confidence", skip_serializing_if = "Option::is_none")]
    pub label_confidence: Option<String>,
    /// Sample MD5
    #[serde(rename = "md5", skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// Search pattern
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Search pattern type
    #[serde(rename = "pattern_type", skip_serializing_if = "Option::is_none")]
    pub pattern_type: Option<String>,
    /// List of sample metadata
    #[serde(rename = "samples")]
    pub samples: Vec<models::MalqueryPeriodSampleMetadata>,
    /// Sample SHA1
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    /// Sample SHA256
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// List of resource tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Search YARA rule
    #[serde(rename = "yara_rule", skip_serializing_if = "Option::is_none")]
    pub yara_rule: Option<String>,
    /// List of YARA rules for related files
    #[serde(rename = "yara_rules", skip_serializing_if = "Option::is_none")]
    pub yara_rules: Option<Vec<String>>,
}

impl MalqueryPeriodExternalResource {
    pub fn new(
        samples: Vec<models::MalqueryPeriodSampleMetadata>,
    ) -> MalqueryPeriodExternalResource {
        MalqueryPeriodExternalResource {
            family: None,
            filesize: None,
            filetype: None,
            first_seen: None,
            ignore_reason: None,
            label: None,
            label_confidence: None,
            md5: None,
            pattern: None,
            pattern_type: None,
            samples,
            sha1: None,
            sha256: None,
            tags: None,
            yara_rule: None,
            yara_rules: None,
        }
    }
}
