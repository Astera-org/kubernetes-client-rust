/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ModifyVolumeStatus : ModifyVolumeStatus represents the status object of ControllerModifyVolume operation



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ModifyVolumeStatus {
    /// status is the status of the ControllerModifyVolume operation. It can be in any of following states:  - Pending    Pending indicates that the PersistentVolumeClaim cannot be modified due to unmet requirements, such as    the specified VolumeAttributesClass not existing.  - InProgress    InProgress indicates that the volume is being modified.  - Infeasible   Infeasible indicates that the request has been rejected as invalid by the CSI driver. To    resolve the error, a valid VolumeAttributesClass needs to be specified. Note: New statuses can be added in the future. Consumers should check for unknown statuses and fail appropriately.
    #[serde(rename = "status")]
    pub status: String,
    /// targetVolumeAttributesClassName is the name of the VolumeAttributesClass the PVC currently being reconciled
    #[serde(rename = "targetVolumeAttributesClassName", skip_serializing_if = "Option::is_none")]
    pub target_volume_attributes_class_name: Option<String>,
}

impl V1ModifyVolumeStatus {
    /// ModifyVolumeStatus represents the status object of ControllerModifyVolume operation
    pub fn new(status: String) -> V1ModifyVolumeStatus {
        V1ModifyVolumeStatus {
            status,
            target_volume_attributes_class_name: None,
        }
    }
}


