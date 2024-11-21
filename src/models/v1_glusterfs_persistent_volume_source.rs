/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1GlusterfsPersistentVolumeSource : Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1GlusterfsPersistentVolumeSource {
    /// endpoints is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(rename = "endpoints")]
    pub endpoints: String,
    /// endpointsNamespace is the namespace that contains Glusterfs endpoint. If this field is empty, the EndpointNamespace defaults to the same namespace as the bound PVC. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(rename = "endpointsNamespace", skip_serializing_if = "Option::is_none")]
    pub endpoints_namespace: Option<String>,
    /// path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(rename = "path")]
    pub path: String,
    /// readOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl V1GlusterfsPersistentVolumeSource {
    /// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.
    pub fn new(endpoints: String, path: String) -> V1GlusterfsPersistentVolumeSource {
        V1GlusterfsPersistentVolumeSource {
            endpoints,
            endpoints_namespace: None,
            path,
            read_only: None,
        }
    }
}


