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
pub struct DeviceapiPeriodDeviceSwagger {
    #[serde(rename = "agent_load_flags", skip_serializing_if = "Option::is_none")]
    pub agent_load_flags: Option<String>,
    #[serde(rename = "agent_local_time", skip_serializing_if = "Option::is_none")]
    pub agent_local_time: Option<String>,
    #[serde(rename = "agent_version", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "base_image_version", skip_serializing_if = "Option::is_none")]
    pub base_image_version: Option<String>,
    #[serde(rename = "bios_manufacturer", skip_serializing_if = "Option::is_none")]
    pub bios_manufacturer: Option<String>,
    #[serde(rename = "bios_version", skip_serializing_if = "Option::is_none")]
    pub bios_version: Option<String>,
    #[serde(rename = "build_number", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    #[serde(rename = "chassis_type", skip_serializing_if = "Option::is_none")]
    pub chassis_type: Option<String>,
    #[serde(rename = "chassis_type_desc", skip_serializing_if = "Option::is_none")]
    pub chassis_type_desc: Option<String>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "config_id_base", skip_serializing_if = "Option::is_none")]
    pub config_id_base: Option<String>,
    #[serde(rename = "config_id_build", skip_serializing_if = "Option::is_none")]
    pub config_id_build: Option<String>,
    #[serde(rename = "config_id_platform", skip_serializing_if = "Option::is_none")]
    pub config_id_platform: Option<String>,
    #[serde(rename = "connection_ip", skip_serializing_if = "Option::is_none")]
    pub connection_ip: Option<String>,
    #[serde(
        rename = "connection_mac_address",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_mac_address: Option<String>,
    #[serde(rename = "cpu_signature", skip_serializing_if = "Option::is_none")]
    pub cpu_signature: Option<String>,
    #[serde(rename = "cpu_vendor", skip_serializing_if = "Option::is_none")]
    pub cpu_vendor: Option<String>,
    #[serde(rename = "default_gateway_ip", skip_serializing_if = "Option::is_none")]
    pub default_gateway_ip: Option<String>,
    #[serde(rename = "deployment_type", skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(
        rename = "detection_suppression_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub detection_suppression_status: Option<String>,
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "device_policies", skip_serializing_if = "Option::is_none")]
    pub device_policies: Option<Box<models::DevicePeriodMappedDevicePolicies>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "external_ip", skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[serde(
        rename = "filesystem_containment_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub filesystem_containment_status: Option<String>,
    #[serde(
        rename = "first_login_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_login_timestamp: Option<String>,
    #[serde(rename = "first_seen", skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    #[serde(rename = "group_hash", skip_serializing_if = "Option::is_none")]
    pub group_hash: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "host_hidden_status", skip_serializing_if = "Option::is_none")]
    pub host_hidden_status: Option<String>,
    #[serde(rename = "host_utc_offset", skip_serializing_if = "Option::is_none")]
    pub host_utc_offset: Option<String>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "internet_exposure", skip_serializing_if = "Option::is_none")]
    pub internet_exposure: Option<String>,
    #[serde(
        rename = "k8s_cluster_git_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_cluster_git_version: Option<String>,
    #[serde(rename = "k8s_cluster_id", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    #[serde(
        rename = "k8s_cluster_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_cluster_version: Option<String>,
    #[serde(rename = "kernel_version", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[serde(
        rename = "last_login_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login_timestamp: Option<String>,
    #[serde(rename = "last_login_uid", skip_serializing_if = "Option::is_none")]
    pub last_login_uid: Option<String>,
    #[serde(rename = "last_login_user", skip_serializing_if = "Option::is_none")]
    pub last_login_user: Option<String>,
    #[serde(
        rename = "last_login_user_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login_user_sid: Option<String>,
    #[serde(rename = "last_reboot", skip_serializing_if = "Option::is_none")]
    pub last_reboot: Option<String>,
    #[serde(rename = "last_seen", skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    #[serde(rename = "linux_sensor_mode", skip_serializing_if = "Option::is_none")]
    pub linux_sensor_mode: Option<String>,
    #[serde(rename = "local_ip", skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "machine_domain", skip_serializing_if = "Option::is_none")]
    pub machine_domain: Option<String>,
    #[serde(rename = "major_version", skip_serializing_if = "Option::is_none")]
    pub major_version: Option<String>,
    #[serde(rename = "managed_apps", skip_serializing_if = "Option::is_none")]
    pub managed_apps: Option<Box<models::DevicePeriodManagedApps>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::DevicePeriodDeviceMeta>>,
    #[serde(
        rename = "migration_completed_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub migration_completed_time: Option<String>,
    #[serde(rename = "minor_version", skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    #[serde(rename = "modified_timestamp", skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    #[serde(rename = "os_build", skip_serializing_if = "Option::is_none")]
    pub os_build: Option<String>,
    #[serde(rename = "os_product_name", skip_serializing_if = "Option::is_none")]
    pub os_product_name: Option<String>,
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "ou", skip_serializing_if = "Option::is_none")]
    pub ou: Option<Vec<String>>,
    #[serde(rename = "platform_id", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "platform_name", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "pod_annotations", skip_serializing_if = "Option::is_none")]
    pub pod_annotations: Option<Vec<String>>,
    #[serde(rename = "pod_host_ip4", skip_serializing_if = "Option::is_none")]
    pub pod_host_ip4: Option<String>,
    #[serde(rename = "pod_host_ip6", skip_serializing_if = "Option::is_none")]
    pub pod_host_ip6: Option<String>,
    #[serde(rename = "pod_hostname", skip_serializing_if = "Option::is_none")]
    pub pod_hostname: Option<String>,
    #[serde(rename = "pod_id", skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    #[serde(rename = "pod_ip4", skip_serializing_if = "Option::is_none")]
    pub pod_ip4: Option<String>,
    #[serde(rename = "pod_ip6", skip_serializing_if = "Option::is_none")]
    pub pod_ip6: Option<String>,
    #[serde(rename = "pod_labels", skip_serializing_if = "Option::is_none")]
    pub pod_labels: Option<Vec<String>>,
    #[serde(rename = "pod_name", skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
    #[serde(rename = "pod_namespace", skip_serializing_if = "Option::is_none")]
    pub pod_namespace: Option<String>,
    #[serde(
        rename = "pod_service_account_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_service_account_name: Option<String>,
    #[serde(rename = "pointer_size", skip_serializing_if = "Option::is_none")]
    pub pointer_size: Option<String>,
    #[serde(rename = "policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(rename = "product_type_desc", skip_serializing_if = "Option::is_none")]
    pub product_type_desc: Option<String>,
    #[serde(rename = "provision_status", skip_serializing_if = "Option::is_none")]
    pub provision_status: Option<String>,
    #[serde(
        rename = "reduced_functionality_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub reduced_functionality_mode: Option<String>,
    #[serde(rename = "release_group", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<String>,
    #[serde(rename = "rtr_state", skip_serializing_if = "Option::is_none")]
    pub rtr_state: Option<String>,
    #[serde(rename = "serial_number", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "service_pack_major", skip_serializing_if = "Option::is_none")]
    pub service_pack_major: Option<String>,
    #[serde(rename = "service_pack_minor", skip_serializing_if = "Option::is_none")]
    pub service_pack_minor: Option<String>,
    #[serde(rename = "service_provider", skip_serializing_if = "Option::is_none")]
    pub service_provider: Option<String>,
    #[serde(
        rename = "service_provider_account_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_provider_account_id: Option<String>,
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        rename = "system_manufacturer",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_manufacturer: Option<String>,
    #[serde(
        rename = "system_product_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_product_name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "zone_group", skip_serializing_if = "Option::is_none")]
    pub zone_group: Option<String>,
}

impl DeviceapiPeriodDeviceSwagger {
    pub fn new(cid: String, device_id: String) -> DeviceapiPeriodDeviceSwagger {
        DeviceapiPeriodDeviceSwagger {
            agent_load_flags: None,
            agent_local_time: None,
            agent_version: None,
            base_image_version: None,
            bios_manufacturer: None,
            bios_version: None,
            build_number: None,
            chassis_type: None,
            chassis_type_desc: None,
            cid,
            config_id_base: None,
            config_id_build: None,
            config_id_platform: None,
            connection_ip: None,
            connection_mac_address: None,
            cpu_signature: None,
            cpu_vendor: None,
            default_gateway_ip: None,
            deployment_type: None,
            detection_suppression_status: None,
            device_id,
            device_policies: None,
            email: None,
            external_ip: None,
            filesystem_containment_status: None,
            first_login_timestamp: None,
            first_seen: None,
            group_hash: None,
            groups: None,
            host_hidden_status: None,
            host_utc_offset: None,
            hostname: None,
            instance_id: None,
            internet_exposure: None,
            k8s_cluster_git_version: None,
            k8s_cluster_id: None,
            k8s_cluster_version: None,
            kernel_version: None,
            last_login_timestamp: None,
            last_login_uid: None,
            last_login_user: None,
            last_login_user_sid: None,
            last_reboot: None,
            last_seen: None,
            linux_sensor_mode: None,
            local_ip: None,
            mac_address: None,
            machine_domain: None,
            major_version: None,
            managed_apps: None,
            meta: None,
            migration_completed_time: None,
            minor_version: None,
            modified_timestamp: None,
            notes: None,
            os_build: None,
            os_product_name: None,
            os_version: None,
            ou: None,
            platform_id: None,
            platform_name: None,
            pod_annotations: None,
            pod_host_ip4: None,
            pod_host_ip6: None,
            pod_hostname: None,
            pod_id: None,
            pod_ip4: None,
            pod_ip6: None,
            pod_labels: None,
            pod_name: None,
            pod_namespace: None,
            pod_service_account_name: None,
            pointer_size: None,
            policies: None,
            product_type: None,
            product_type_desc: None,
            provision_status: None,
            reduced_functionality_mode: None,
            release_group: None,
            rtr_state: None,
            serial_number: None,
            service_pack_major: None,
            service_pack_minor: None,
            service_provider: None,
            service_provider_account_id: None,
            site_name: None,
            status: None,
            system_manufacturer: None,
            system_product_name: None,
            tags: None,
            zone_group: None,
        }
    }
}
