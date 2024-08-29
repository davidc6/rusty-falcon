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
pub struct ModelsPeriodApiKubernetesIom {
    #[serde(
        rename = "admission_review_action",
        skip_serializing_if = "Option::is_none"
    )]
    pub admission_review_action: Option<String>,
    #[serde(
        rename = "admission_review_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub admission_review_id: Option<String>,
    #[serde(
        rename = "admission_review_msg",
        skip_serializing_if = "Option::is_none"
    )]
    pub admission_review_msg: Option<String>,
    #[serde(
        rename = "admission_review_operation",
        skip_serializing_if = "Option::is_none"
    )]
    pub admission_review_operation: Option<String>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cis_id")]
    pub cis_id: Vec<String>,
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
    #[serde(rename = "cluster_name")]
    pub cluster_name: String,
    #[serde(rename = "containers_impacted_count")]
    pub containers_impacted_count: String,
    #[serde(rename = "containers_impacted_ids")]
    pub containers_impacted_ids: Vec<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "detect_timestamp")]
    pub detect_timestamp: String,
    #[serde(rename = "detection_id")]
    pub detection_id: String,
    #[serde(rename = "detection_name")]
    pub detection_name: String,
    #[serde(rename = "detection_type")]
    pub detection_type: String,
    #[serde(
        rename = "image_assessment_matched_cves",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_assessment_matched_cves: Option<Vec<String>>,
    #[serde(
        rename = "image_assessment_policy_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_assessment_policy_description: Option<String>,
    #[serde(
        rename = "image_assessment_policy_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_assessment_policy_id: Option<String>,
    #[serde(
        rename = "image_assessment_policy_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_assessment_policy_name: Option<String>,
    #[serde(rename = "image_digest", skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(
        rename = "image_has_been_assessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_has_been_assessed: Option<bool>,
    #[serde(rename = "image_id", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "image_registry", skip_serializing_if = "Option::is_none")]
    pub image_registry: Option<String>,
    #[serde(rename = "image_repository", skip_serializing_if = "Option::is_none")]
    pub image_repository: Option<String>,
    #[serde(rename = "image_tag", skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[serde(
        rename = "last_seen_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_seen_timestamp: Option<String>,
    #[serde(rename = "mitigation_id", skip_serializing_if = "Option::is_none")]
    pub mitigation_id: Option<String>,
    #[serde(rename = "mitigation_name", skip_serializing_if = "Option::is_none")]
    pub mitigation_name: Option<String>,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "nist_id", skip_serializing_if = "Option::is_none")]
    pub nist_id: Option<String>,
    #[serde(rename = "pod_label", skip_serializing_if = "Option::is_none")]
    pub pod_label: Option<String>,
    #[serde(
        rename = "pods_impacted_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub pods_impacted_count: Option<String>,
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "policy_name", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(rename = "prevented")]
    pub prevented: String,
    #[serde(rename = "remediation")]
    pub remediation: String,
    #[serde(rename = "resource_creation_timestamp")]
    pub resource_creation_timestamp: String,
    #[serde(
        rename = "resource_group_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    #[serde(rename = "resource_name")]
    pub resource_name: String,
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    #[serde(rename = "sensitive_env_vars")]
    pub sensitive_env_vars: Vec<String>,
    #[serde(rename = "service_type", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "tactic_id")]
    pub tactic_id: String,
    #[serde(rename = "tactic_name")]
    pub tactic_name: String,
    #[serde(rename = "technique_id")]
    pub technique_id: String,
    #[serde(rename = "technique_name")]
    pub technique_name: String,
}

impl ModelsPeriodApiKubernetesIom {
    pub fn new(
        cid: String,
        cis_id: Vec<String>,
        cluster_id: String,
        cluster_name: String,
        containers_impacted_count: String,
        containers_impacted_ids: Vec<String>,
        description: String,
        detect_timestamp: String,
        detection_id: String,
        detection_name: String,
        detection_type: String,
        namespace: String,
        prevented: String,
        remediation: String,
        resource_creation_timestamp: String,
        resource_id: String,
        resource_name: String,
        resource_type: String,
        sensitive_env_vars: Vec<String>,
        severity: String,
        tactic_id: String,
        tactic_name: String,
        technique_id: String,
        technique_name: String,
    ) -> ModelsPeriodApiKubernetesIom {
        ModelsPeriodApiKubernetesIom {
            admission_review_action: None,
            admission_review_id: None,
            admission_review_msg: None,
            admission_review_operation: None,
            cid,
            cis_id,
            cluster_id,
            cluster_name,
            containers_impacted_count,
            containers_impacted_ids,
            description,
            detect_timestamp,
            detection_id,
            detection_name,
            detection_type,
            image_assessment_matched_cves: None,
            image_assessment_policy_description: None,
            image_assessment_policy_id: None,
            image_assessment_policy_name: None,
            image_digest: None,
            image_has_been_assessed: None,
            image_id: None,
            image_registry: None,
            image_repository: None,
            image_tag: None,
            last_seen_timestamp: None,
            mitigation_id: None,
            mitigation_name: None,
            namespace,
            nist_id: None,
            pod_label: None,
            pods_impacted_count: None,
            policy_id: None,
            policy_name: None,
            ports: None,
            prevented,
            remediation,
            resource_creation_timestamp,
            resource_group_name: None,
            resource_id,
            resource_name,
            resource_type,
            sensitive_env_vars,
            service_type: None,
            severity,
            tactic_id,
            tactic_name,
            technique_id,
            technique_name,
        }
    }
}
