/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1FlexVolumeSource : FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1FlexVolumeSource {
    /// driver is the name of the driver to use for this volume.
    #[serde(rename = "driver")]
    pub driver: String,
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default filesystem depends on FlexVolume script.
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// options is Optional: this field holds extra command options if any.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<Box<crate::models::V1LocalObjectReference>>,
}

impl V1FlexVolumeSource {
    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    pub fn new(driver: String) -> V1FlexVolumeSource {
        V1FlexVolumeSource {
            driver,
            fs_type: None,
            options: None,
            read_only: None,
            secret_ref: None,
        }
    }
}


