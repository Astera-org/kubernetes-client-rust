/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1EmptyDirVolumeSource : Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1EmptyDirVolumeSource {
    /// medium represents what type of storage medium should back this directory. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(rename = "medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// sizeLimit is the total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(rename = "sizeLimit", skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<String>,
}

impl V1EmptyDirVolumeSource {
    /// Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.
    pub fn new() -> V1EmptyDirVolumeSource {
        V1EmptyDirVolumeSource {
            medium: None,
            size_limit: None,
        }
    }
}


