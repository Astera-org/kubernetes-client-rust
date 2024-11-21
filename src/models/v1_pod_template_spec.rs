/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PodTemplateSpec : PodTemplateSpec describes the data a pod should have when created from a template



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1PodTemplateSpec {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ObjectMeta>>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::V1PodSpec>>,
}

impl V1PodTemplateSpec {
    /// PodTemplateSpec describes the data a pod should have when created from a template
    pub fn new() -> V1PodTemplateSpec {
        V1PodTemplateSpec {
            metadata: None,
            spec: None,
        }
    }
}


