/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Overhead : Overhead structure represents the resource overhead associated with running a pod.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1Overhead {
    /// podFixed represents the fixed resource overhead associated with running a pod.
    #[serde(rename = "podFixed", skip_serializing_if = "Option::is_none")]
    pub pod_fixed: Option<::std::collections::HashMap<String, String>>,
}

impl V1Overhead {
    /// Overhead structure represents the resource overhead associated with running a pod.
    pub fn new() -> V1Overhead {
        V1Overhead {
            pod_fixed: None,
        }
    }
}


