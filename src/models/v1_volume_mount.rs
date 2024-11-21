/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1VolumeMount : VolumeMount describes a mounting of a Volume within a container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1VolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10. When RecursiveReadOnly is set to IfPossible or to Enabled, MountPropagation must be None or unspecified (which defaults to None).
    #[serde(rename = "mountPropagation", skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// This must match the Name of a Volume.
    #[serde(rename = "name")]
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// RecursiveReadOnly specifies whether read-only mounts should be handled recursively.  If ReadOnly is false, this field has no meaning and must be unspecified.  If ReadOnly is true, and this field is set to Disabled, the mount is not made recursively read-only.  If this field is set to IfPossible, the mount is made recursively read-only, if it is supported by the container runtime.  If this field is set to Enabled, the mount is made recursively read-only if it is supported by the container runtime, otherwise the pod will not be started and an error will be generated to indicate the reason.  If this field is set to IfPossible or Enabled, MountPropagation must be set to None (or be unspecified, which defaults to None).  If this field is not specified, it is treated as an equivalent of Disabled.
    #[serde(rename = "recursiveReadOnly", skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<String>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root).
    #[serde(rename = "subPath", skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to \"\" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(rename = "subPathExpr", skip_serializing_if = "Option::is_none")]
    pub sub_path_expr: Option<String>,
}

impl V1VolumeMount {
    /// VolumeMount describes a mounting of a Volume within a container.
    pub fn new(mount_path: String, name: String) -> V1VolumeMount {
        V1VolumeMount {
            mount_path,
            mount_propagation: None,
            name,
            read_only: None,
            recursive_read_only: None,
            sub_path: None,
            sub_path_expr: None,
        }
    }
}

