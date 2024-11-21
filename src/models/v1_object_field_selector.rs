/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ObjectFieldSelector : ObjectFieldSelector selects an APIVersioned field of an object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of, defaults to \"v1\".
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

impl V1ObjectFieldSelector {
    /// ObjectFieldSelector selects an APIVersioned field of an object.
    pub fn new(field_path: String) -> V1ObjectFieldSelector {
        V1ObjectFieldSelector {
            api_version: None,
            field_path,
        }
    }
}

