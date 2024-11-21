/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NodeStatus : NodeStatus is information about the current status of a node.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NodeStatus {
    /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::V1NodeAddress>>,
    /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    #[serde(rename = "allocatable", skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<::std::collections::HashMap<String, String>>,
    /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<::std::collections::HashMap<String, String>>,
    /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1NodeCondition>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::V1NodeConfigStatus>>,
    #[serde(rename = "daemonEndpoints", skip_serializing_if = "Option::is_none")]
    pub daemon_endpoints: Option<Box<crate::models::V1NodeDaemonEndpoints>>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Box<crate::models::V1NodeFeatures>>,
    /// List of container images on this node
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::V1ContainerImage>>,
    #[serde(rename = "nodeInfo", skip_serializing_if = "Option::is_none")]
    pub node_info: Option<Box<crate::models::V1NodeSystemInfo>>,
    /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The available runtime handlers.
    #[serde(rename = "runtimeHandlers", skip_serializing_if = "Option::is_none")]
    pub runtime_handlers: Option<Vec<crate::models::V1NodeRuntimeHandler>>,
    /// List of volumes that are attached to the node.
    #[serde(rename = "volumesAttached", skip_serializing_if = "Option::is_none")]
    pub volumes_attached: Option<Vec<crate::models::V1AttachedVolume>>,
    /// List of attachable volumes in use (mounted) by the node.
    #[serde(rename = "volumesInUse", skip_serializing_if = "Option::is_none")]
    pub volumes_in_use: Option<Vec<String>>,
}

impl V1NodeStatus {
    /// NodeStatus is information about the current status of a node.
    pub fn new() -> V1NodeStatus {
        V1NodeStatus {
            addresses: None,
            allocatable: None,
            capacity: None,
            conditions: None,
            config: None,
            daemon_endpoints: None,
            features: None,
            images: None,
            node_info: None,
            phase: None,
            runtime_handlers: None,
            volumes_attached: None,
            volumes_in_use: None,
        }
    }
}

