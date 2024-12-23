/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ConfigMapNodeConfigSource : ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ConfigMapNodeConfigSource {
    /// KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
    #[serde(rename = "kubeletConfigKey")]
    pub kubelet_config_key: String,
    /// Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
    #[serde(rename = "name")]
    pub name: String,
    /// Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    #[serde(rename = "resourceVersion", skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
    /// UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl V1ConfigMapNodeConfigSource {
    /// ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration
    pub fn new(kubelet_config_key: String, name: String, namespace: String) -> V1ConfigMapNodeConfigSource {
        V1ConfigMapNodeConfigSource {
            kubelet_config_key,
            name,
            namespace,
            resource_version: None,
            uid: None,
        }
    }
}


