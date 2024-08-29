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
pub struct ModelsPeriodComplianceExportGroupedByRulesReport {
    #[serde(rename = "asset_type")]
    pub asset_type: String,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "clusters")]
    pub clusters: i64,
    #[serde(rename = "clusters_list")]
    pub clusters_list: String,
    #[serde(rename = "failed_containers")]
    pub failed_containers: i64,
    #[serde(rename = "failed_images")]
    pub failed_images: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passed_containers")]
    pub passed_containers: i64,
    #[serde(rename = "passed_images")]
    pub passed_images: i64,
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    #[serde(rename = "severity")]
    pub severity: f64,
    #[serde(rename = "total_containers")]
    pub total_containers: i64,
    #[serde(rename = "total_images")]
    pub total_images: i64,
}

impl ModelsPeriodComplianceExportGroupedByRulesReport {
    pub fn new(
        asset_type: String,
        authority: String,
        clusters: i64,
        clusters_list: String,
        failed_containers: i64,
        failed_images: i64,
        name: String,
        passed_containers: i64,
        passed_images: i64,
        rule_id: String,
        severity: f64,
        total_containers: i64,
        total_images: i64,
    ) -> ModelsPeriodComplianceExportGroupedByRulesReport {
        ModelsPeriodComplianceExportGroupedByRulesReport {
            asset_type,
            authority,
            clusters,
            clusters_list,
            failed_containers,
            failed_images,
            name,
            passed_containers,
            passed_images,
            rule_id,
            severity,
            total_containers,
            total_images,
        }
    }
}
