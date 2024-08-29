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
pub struct DomainPeriodTweetSource {
    /// The username of the tweet's author
    #[serde(rename = "author_name")]
    pub author_name: String,
    /// The language of the tweet
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "legacy_source", skip_serializing_if = "Option::is_none")]
    pub legacy_source: Option<serde_json::Value>,
    /// The link to the tweet
    #[serde(rename = "source_link")]
    pub source_link: String,
    /// The tweet ID
    #[serde(rename = "tweet_id")]
    pub tweet_id: i64,
}

impl DomainPeriodTweetSource {
    pub fn new(
        author_name: String,
        language: String,
        source_link: String,
        tweet_id: i64,
    ) -> DomainPeriodTweetSource {
        DomainPeriodTweetSource {
            author_name,
            language,
            legacy_source: None,
            source_link,
            tweet_id,
        }
    }
}
