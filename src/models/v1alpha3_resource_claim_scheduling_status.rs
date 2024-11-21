/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3ResourceClaimSchedulingStatus : ResourceClaimSchedulingStatus contains information about one particular ResourceClaim with \"WaitForFirstConsumer\" allocation mode.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3ResourceClaimSchedulingStatus {
    /// Name matches the pod.spec.resourceClaims[*].Name field.
    #[serde(rename = "name")]
    pub name: String,
    /// UnsuitableNodes lists nodes that the ResourceClaim cannot be allocated for.  The size of this field is limited to 128, the same as for PodSchedulingSpec.PotentialNodes. This may get increased in the future, but not reduced.
    #[serde(rename = "unsuitableNodes", skip_serializing_if = "Option::is_none")]
    pub unsuitable_nodes: Option<Vec<String>>,
}

impl V1alpha3ResourceClaimSchedulingStatus {
    /// ResourceClaimSchedulingStatus contains information about one particular ResourceClaim with \"WaitForFirstConsumer\" allocation mode.
    pub fn new(name: String) -> V1alpha3ResourceClaimSchedulingStatus {
        V1alpha3ResourceClaimSchedulingStatus {
            name,
            unsuitable_nodes: None,
        }
    }
}

