/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PodDisruptionBudgetSpec : PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1PodDisruptionBudgetSpec {
    /// An eviction is allowed if at most \"maxUnavailable\" pods selected by \"selector\" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with \"minAvailable\".
    #[serde(rename = "maxUnavailable", skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<serde_json::Value>,
    /// An eviction is allowed if at least \"minAvailable\" pods selected by \"selector\" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying \"100%\".
    #[serde(rename = "minAvailable", skip_serializing_if = "Option::is_none")]
    pub min_available: Option<serde_json::Value>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::models::V1LabelSelector>>,
    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction. Current implementation considers healthy pods, as pods that have status.conditions item with type=\"Ready\",status=\"True\".  Valid policies are IfHealthyBudget and AlwaysAllow. If no policy is specified, the default behavior will be used, which corresponds to the IfHealthyBudget policy.  IfHealthyBudget policy means that running pods (status.phase=\"Running\"), but not yet healthy can be evicted only if the guarded application is not disrupted (status.currentHealthy is at least equal to status.desiredHealthy). Healthy pods will be subject to the PDB for eviction.  AlwaysAllow policy means that all running pods (status.phase=\"Running\"), but not yet healthy are considered disrupted and can be evicted regardless of whether the criteria in a PDB is met. This means perspective running pods of a disrupted application might not get a chance to become healthy. Healthy pods will be subject to the PDB for eviction.  Additional policies may be added in the future. Clients making eviction decisions should disallow eviction of unhealthy pods if they encounter an unrecognized policy in this field.  This field is beta-level. The eviction API uses this field when the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).
    #[serde(rename = "unhealthyPodEvictionPolicy", skip_serializing_if = "Option::is_none")]
    pub unhealthy_pod_eviction_policy: Option<String>,
}

impl V1PodDisruptionBudgetSpec {
    /// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
    pub fn new() -> V1PodDisruptionBudgetSpec {
        V1PodDisruptionBudgetSpec {
            max_unavailable: None,
            min_available: None,
            selector: None,
            unhealthy_pod_eviction_policy: None,
        }
    }
}


