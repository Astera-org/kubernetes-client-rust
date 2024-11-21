/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CinderVolumeSource : Represents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CinderVolumeSource {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<Box<crate::models::V1LocalObjectReference>>,
    /// volumeID used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(rename = "volumeID")]
    pub volume_id: String,
}

impl V1CinderVolumeSource {
    /// Represents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling.
    pub fn new(volume_id: String) -> V1CinderVolumeSource {
        V1CinderVolumeSource {
            fs_type: None,
            read_only: None,
            secret_ref: None,
            volume_id,
        }
    }
}


