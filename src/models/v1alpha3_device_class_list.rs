/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3DeviceClassList : DeviceClassList is a collection of classes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3DeviceClassList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Items is the list of resource classes.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::V1alpha3DeviceClass>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ListMeta>>,
}

impl V1alpha3DeviceClassList {
    /// DeviceClassList is a collection of classes.
    pub fn new(items: Vec<crate::models::V1alpha3DeviceClass>) -> V1alpha3DeviceClassList {
        V1alpha3DeviceClassList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}


