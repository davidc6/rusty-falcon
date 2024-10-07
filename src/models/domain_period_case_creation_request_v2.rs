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
pub struct DomainPeriodCaseCreationRequestV2 {
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "detections")]
    pub detections: Vec<models::MessagesPeriodAlert>,
    #[serde(rename = "incidents")]
    pub incidents: Vec<models::MessagesPeriodIncident>,
    #[serde(rename = "malware_submission_id")]
    pub malware_submission_id: String,
    #[serde(rename = "recon_rule_type")]
    pub recon_rule_type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "user_uuid", skip_serializing_if = "Option::is_none")]
    pub user_uuid: Option<String>,
}

impl DomainPeriodCaseCreationRequestV2 {
    pub fn new(
        body: String,
        detections: Vec<models::MessagesPeriodAlert>,
        incidents: Vec<models::MessagesPeriodIncident>,
        malware_submission_id: String,
        recon_rule_type: String,
        title: String,
        r#type: String,
    ) -> DomainPeriodCaseCreationRequestV2 {
        DomainPeriodCaseCreationRequestV2 {
            body,
            detections,
            incidents,
            malware_submission_id,
            recon_rule_type,
            title,
            r#type,
            user_uuid: None,
        }
    }
}
