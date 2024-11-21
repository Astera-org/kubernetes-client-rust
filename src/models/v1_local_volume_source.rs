/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1LocalVolumeSource : Local represents directly-attached storage with node affinity (Beta feature)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LocalVolumeSource {
    /// fsType is the filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". The default value is to auto-select a filesystem if unspecified.
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// path of the full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
    #[serde(rename = "path")]
    pub path: String,
}

impl V1LocalVolumeSource {
    /// Local represents directly-attached storage with node affinity (Beta feature)
    pub fn new(path: String) -> V1LocalVolumeSource {
        V1LocalVolumeSource {
            fs_type: None,
            path,
        }
    }
}


