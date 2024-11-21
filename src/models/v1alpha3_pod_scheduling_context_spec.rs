/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3PodSchedulingContextSpec : PodSchedulingContextSpec describes where resources for the Pod are needed.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3PodSchedulingContextSpec {
    /// PotentialNodes lists nodes where the Pod might be able to run.  The size of this field is limited to 128. This is large enough for many clusters. Larger clusters may need more attempts to find a node that suits all pending resources. This may get increased in the future, but not reduced.
    #[serde(rename = "potentialNodes", skip_serializing_if = "Option::is_none")]
    pub potential_nodes: Option<Vec<String>>,
    /// SelectedNode is the node for which allocation of ResourceClaims that are referenced by the Pod and that use \"WaitForFirstConsumer\" allocation is to be attempted.
    #[serde(rename = "selectedNode", skip_serializing_if = "Option::is_none")]
    pub selected_node: Option<String>,
}

impl V1alpha3PodSchedulingContextSpec {
    /// PodSchedulingContextSpec describes where resources for the Pod are needed.
    pub fn new() -> V1alpha3PodSchedulingContextSpec {
        V1alpha3PodSchedulingContextSpec {
            potential_nodes: None,
            selected_node: None,
        }
    }
}


