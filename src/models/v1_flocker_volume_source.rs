/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1FlockerVolumeSource : Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1FlockerVolumeSource {
    /// datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
    #[serde(rename = "datasetName", skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    /// datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
    #[serde(rename = "datasetUUID", skip_serializing_if = "Option::is_none")]
    pub dataset_uuid: Option<String>,
}

impl V1FlockerVolumeSource {
    /// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
    pub fn new() -> V1FlockerVolumeSource {
        V1FlockerVolumeSource {
            dataset_name: None,
            dataset_uuid: None,
        }
    }
}

