/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1VolumeResourceRequirements : VolumeResourceRequirements describes the storage resource requirements for a volume.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1VolumeResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<::std::collections::HashMap<String, String>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<::std::collections::HashMap<String, String>>,
}

impl V1VolumeResourceRequirements {
    /// VolumeResourceRequirements describes the storage resource requirements for a volume.
    pub fn new() -> V1VolumeResourceRequirements {
        V1VolumeResourceRequirements {
            limits: None,
            requests: None,
        }
    }
}


