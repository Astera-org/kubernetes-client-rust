/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1SeccompProfile : SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1SeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must be set if type is \"Localhost\". Must NOT be set for any other type.
    #[serde(rename = "localhostProfile", skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are:  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1SeccompProfile {
    /// SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.
    pub fn new(_type: String) -> V1SeccompProfile {
        V1SeccompProfile {
            localhost_profile: None,
            _type,
        }
    }
}


