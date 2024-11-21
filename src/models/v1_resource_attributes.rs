/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ResourceAttributes : ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ResourceAttributes {
    #[serde(rename = "fieldSelector", skip_serializing_if = "Option::is_none")]
    pub field_selector: Option<Box<crate::models::V1FieldSelectorAttributes>>,
    /// Group is the API Group of the Resource.  \"*\" means all.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "labelSelector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::V1LabelSelectorAttributes>>,
    /// Name is the name of the resource being requested for a \"get\" or deleted for a \"delete\". \"\" (empty) means all.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \"\" (empty) is defaulted for LocalSubjectAccessReviews \"\" (empty) is empty for cluster-scoped resources \"\" (empty) means \"all\" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Resource is one of the existing resource types.  \"*\" means all.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Subresource is one of the existing resource types.  \"\" means none.
    #[serde(rename = "subresource", skip_serializing_if = "Option::is_none")]
    pub subresource: Option<String>,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.
    #[serde(rename = "verb", skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    /// Version is the API Version of the Resource.  \"*\" means all.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl V1ResourceAttributes {
    /// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
    pub fn new() -> V1ResourceAttributes {
        V1ResourceAttributes {
            field_selector: None,
            group: None,
            label_selector: None,
            name: None,
            namespace: None,
            resource: None,
            subresource: None,
            verb: None,
            version: None,
        }
    }
}


