/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ResourceQuotaStatus : ResourceQuotaStatus defines the enforced hard limits and observed use.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    #[serde(rename = "hard", skip_serializing_if = "Option::is_none")]
    pub hard: Option<::std::collections::HashMap<String, String>>,
    /// Used is the current observed total usage of the resource in the namespace.
    #[serde(rename = "used", skip_serializing_if = "Option::is_none")]
    pub used: Option<::std::collections::HashMap<String, String>>,
}

impl V1ResourceQuotaStatus {
    /// ResourceQuotaStatus defines the enforced hard limits and observed use.
    pub fn new() -> V1ResourceQuotaStatus {
        V1ResourceQuotaStatus {
            hard: None,
            used: None,
        }
    }
}


