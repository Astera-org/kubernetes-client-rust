/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NamespaceStatus : NamespaceStatus is information about the current status of a Namespace.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NamespaceStatus {
    /// Represents the latest available observations of a namespace's current state.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1NamespaceCondition>>,
    /// Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

impl V1NamespaceStatus {
    /// NamespaceStatus is information about the current status of a Namespace.
    pub fn new() -> V1NamespaceStatus {
        V1NamespaceStatus {
            conditions: None,
            phase: None,
        }
    }
}


