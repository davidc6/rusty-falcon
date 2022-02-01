/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponsesSensorUpdateKernelV1 {
    #[serde(rename = "architecture")]
    pub architecture: String,
    #[serde(rename = "base_package_supported_sensor_versions")]
    pub base_package_supported_sensor_versions: Vec<String>,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: String,
    #[serde(rename = "distro")]
    pub distro: String,
    #[serde(rename = "distro_version")]
    pub distro_version: String,
    #[serde(rename = "flavor")]
    pub flavor: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "modified_timestamp")]
    pub modified_timestamp: String,
    #[serde(rename = "release")]
    pub release: String,
    #[serde(rename = "vendor")]
    pub vendor: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "ztl_module_supported_sensor_versions")]
    pub ztl_module_supported_sensor_versions: Vec<String>,
    #[serde(rename = "ztl_supported_sensor_versions")]
    pub ztl_supported_sensor_versions: Vec<String>,
}

impl ResponsesSensorUpdateKernelV1 {
    pub fn new(
        architecture: String,
        base_package_supported_sensor_versions: Vec<String>,
        created_timestamp: String,
        distro: String,
        distro_version: String,
        flavor: String,
        id: String,
        modified_timestamp: String,
        release: String,
        vendor: String,
        version: String,
        ztl_module_supported_sensor_versions: Vec<String>,
        ztl_supported_sensor_versions: Vec<String>,
    ) -> ResponsesSensorUpdateKernelV1 {
        ResponsesSensorUpdateKernelV1 {
            architecture,
            base_package_supported_sensor_versions,
            created_timestamp,
            distro,
            distro_version,
            flavor,
            id,
            modified_timestamp,
            release,
            vendor,
            version,
            ztl_module_supported_sensor_versions,
            ztl_supported_sensor_versions,
        }
    }
}
