/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1VolumeNodeResources : VolumeNodeResources is a set of resource limits for scheduling of volumes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1VolumeNodeResources {
    /// count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl V1VolumeNodeResources {
    /// VolumeNodeResources is a set of resource limits for scheduling of volumes.
    pub fn new() -> V1VolumeNodeResources {
        V1VolumeNodeResources {
            count: None,
        }
    }
}


