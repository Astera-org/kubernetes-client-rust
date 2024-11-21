/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NamespaceSpec : NamespaceSpec describes the attributes on a Namespace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    #[serde(rename = "finalizers", skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<Vec<String>>,
}

impl V1NamespaceSpec {
    /// NamespaceSpec describes the attributes on a Namespace.
    pub fn new() -> V1NamespaceSpec {
        V1NamespaceSpec {
            finalizers: None,
        }
    }
}


