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
pub struct ModelPeriodArgument {
    #[serde(rename = "arg_name")]
    pub arg_name: String,
    #[serde(rename = "arg_type")]
    pub arg_type: String,
    #[serde(rename = "command_level")]
    pub command_level: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "data_type")]
    pub data_type: String,
    #[serde(rename = "default_value")]
    pub default_value: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "encoding")]
    pub encoding: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "options")]
    pub options: Vec<String>,
    #[serde(rename = "required")]
    pub required: bool,
    #[serde(rename = "requires_value")]
    pub requires_value: bool,
    #[serde(rename = "script_id")]
    pub script_id: i32,
    #[serde(rename = "sequence")]
    pub sequence: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ModelPeriodArgument {
    pub fn new(
        arg_name: String,
        arg_type: String,
        command_level: String,
        created_at: String,
        data_type: String,
        default_value: String,
        description: String,
        encoding: String,
        id: i32,
        options: Vec<String>,
        required: bool,
        requires_value: bool,
        script_id: i32,
        sequence: i32,
        updated_at: String,
    ) -> ModelPeriodArgument {
        ModelPeriodArgument {
            arg_name,
            arg_type,
            command_level,
            created_at,
            data_type,
            default_value,
            description,
            encoding,
            id,
            options,
            required,
            requires_value,
            script_id,
            sequence,
            updated_at,
        }
    }
}
