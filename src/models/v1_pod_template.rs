/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PodTemplate : PodTemplate describes a template for creating copies of a predefined pod.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1PodTemplate {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ObjectMeta>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::V1PodTemplateSpec>>,
}

impl V1PodTemplate {
    /// PodTemplate describes a template for creating copies of a predefined pod.
    pub fn new() -> V1PodTemplate {
        V1PodTemplate {
            api_version: None,
            kind: None,
            metadata: None,
            template: None,
        }
    }
}


