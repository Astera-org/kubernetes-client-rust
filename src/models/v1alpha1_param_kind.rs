/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1ParamKind : ParamKind is a tuple of Group Kind and Version.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha1ParamKind {
    /// APIVersion is the API group version the resources belong to. In format of \"group/version\". Required.
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is the API kind the resources belong to. Required.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl V1alpha1ParamKind {
    /// ParamKind is a tuple of Group Kind and Version.
    pub fn new() -> V1alpha1ParamKind {
        V1alpha1ParamKind {
            api_version: None,
            kind: None,
        }
    }
}


