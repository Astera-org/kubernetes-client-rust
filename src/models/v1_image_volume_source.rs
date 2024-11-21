/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ImageVolumeSource : ImageVolumeSource represents a image volume resource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ImageVolumeSource {
    /// Policy for pulling OCI objects. Possible values are: Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
    #[serde(rename = "pullPolicy", skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<String>,
    /// Required: Image or artifact reference to be used. Behaves in the same way as pod.spec.containers[*].image. Pull secrets will be assembled in the same way as for the container image by looking up node credentials, SA image pull secrets, and pod spec image pull secrets. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl V1ImageVolumeSource {
    /// ImageVolumeSource represents a image volume resource.
    pub fn new() -> V1ImageVolumeSource {
        V1ImageVolumeSource {
            pull_policy: None,
            reference: None,
        }
    }
}


