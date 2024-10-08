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
pub struct DomainPeriodXlr8Info {
    #[serde(rename = "asset_criticality")]
    pub asset_criticality: String,
    #[serde(rename = "internet_exposed")]
    pub internet_exposed: bool,
    #[serde(rename = "risk_score")]
    pub risk_score: i32,
    #[serde(rename = "risk_score_string")]
    pub risk_score_string: String,
}

impl DomainPeriodXlr8Info {
    pub fn new(
        asset_criticality: String,
        internet_exposed: bool,
        risk_score: i32,
        risk_score_string: String,
    ) -> DomainPeriodXlr8Info {
        DomainPeriodXlr8Info {
            asset_criticality,
            internet_exposed,
            risk_score,
            risk_score_string,
        }
    }
}
