/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1BoundObjectReference : BoundObjectReference is a reference to an object that a token is bound to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1BoundObjectReference {
    /// API version of the referent.
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind of the referent. Valid kinds are 'Pod' and 'Secret'.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UID of the referent.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl V1BoundObjectReference {
    /// BoundObjectReference is a reference to an object that a token is bound to.
    pub fn new() -> V1BoundObjectReference {
        V1BoundObjectReference {
            api_version: None,
            kind: None,
            name: None,
            uid: None,
        }
    }
}


