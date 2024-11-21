/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ReplicationControllerStatus : ReplicationControllerStatus represents the current status of a replication controller.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ReplicationControllerStatus {
    /// The number of available replicas (ready for at least minReadySeconds) for this replication controller.
    #[serde(rename = "availableReplicas", skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// Represents the latest available observations of a replication controller's current state.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1ReplicationControllerCondition>>,
    /// The number of pods that have labels matching the labels of the pod template of the replication controller.
    #[serde(rename = "fullyLabeledReplicas", skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed replication controller.
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The number of ready replicas for this replication controller.
    #[serde(rename = "readyReplicas", skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    #[serde(rename = "replicas")]
    pub replicas: i32,
}

impl V1ReplicationControllerStatus {
    /// ReplicationControllerStatus represents the current status of a replication controller.
    pub fn new(replicas: i32) -> V1ReplicationControllerStatus {
        V1ReplicationControllerStatus {
            available_replicas: None,
            conditions: None,
            fully_labeled_replicas: None,
            observed_generation: None,
            ready_replicas: None,
            replicas,
        }
    }
}


