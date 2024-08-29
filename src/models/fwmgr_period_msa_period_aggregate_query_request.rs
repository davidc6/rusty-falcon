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
pub struct FwmgrPeriodMsaPeriodAggregateQueryRequest {
    #[serde(rename = "date_ranges")]
    pub date_ranges: Vec<models::FwmgrPeriodMsaPeriodDateRangeSpec>,
    #[serde(rename = "exclude")]
    pub exclude: String,
    #[serde(rename = "extended_bounds", skip_serializing_if = "Option::is_none")]
    pub extended_bounds: Option<Box<models::FwmgrPeriodMsaPeriodExtendedBoundsSpec>>,
    #[serde(rename = "field")]
    pub field: String,
    #[serde(rename = "filter")]
    pub filter: String,
    #[serde(rename = "from")]
    pub from: i32,
    #[serde(rename = "include")]
    pub include: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "max_doc_count", skip_serializing_if = "Option::is_none")]
    pub max_doc_count: Option<i64>,
    #[serde(rename = "min_doc_count", skip_serializing_if = "Option::is_none")]
    pub min_doc_count: Option<i64>,
    #[serde(rename = "missing")]
    pub missing: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "q")]
    pub q: String,
    #[serde(rename = "ranges")]
    pub ranges: Vec<models::FwmgrPeriodMsaPeriodRangeSpec>,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "sort")]
    pub sort: String,
    #[serde(rename = "sub_aggregates")]
    pub sub_aggregates: Vec<models::FwmgrPeriodMsaPeriodAggregateQueryRequest>,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl FwmgrPeriodMsaPeriodAggregateQueryRequest {
    pub fn new(
        date_ranges: Vec<models::FwmgrPeriodMsaPeriodDateRangeSpec>,
        exclude: String,
        field: String,
        filter: String,
        from: i32,
        include: String,
        interval: String,
        missing: String,
        name: String,
        q: String,
        ranges: Vec<models::FwmgrPeriodMsaPeriodRangeSpec>,
        size: i32,
        sort: String,
        sub_aggregates: Vec<models::FwmgrPeriodMsaPeriodAggregateQueryRequest>,
        time_zone: String,
        r#type: String,
    ) -> FwmgrPeriodMsaPeriodAggregateQueryRequest {
        FwmgrPeriodMsaPeriodAggregateQueryRequest {
            date_ranges,
            exclude,
            extended_bounds: None,
            field,
            filter,
            from,
            include,
            interval,
            max_doc_count: None,
            min_doc_count: None,
            missing,
            name,
            q,
            ranges,
            size,
            sort,
            sub_aggregates,
            time_zone,
            r#type,
        }
    }
}
