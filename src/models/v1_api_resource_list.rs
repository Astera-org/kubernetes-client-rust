/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ApiResourceList : APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ApiResourceList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// groupVersion is the group and version this APIResourceList is for.
    #[serde(rename = "groupVersion")]
    pub group_version: String,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// resources contains the name of the resources and if they are namespaced.
    #[serde(rename = "resources")]
    pub resources: Vec<crate::models::V1ApiResource>,
}

impl V1ApiResourceList {
    /// APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.
    pub fn new(group_version: String, resources: Vec<crate::models::V1ApiResource>) -> V1ApiResourceList {
        V1ApiResourceList {
            api_version: None,
            group_version,
            kind: None,
            resources,
        }
    }
}


