/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NetworkPolicyPeer : NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NetworkPolicyPeer {
    #[serde(rename = "ipBlock", skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<Box<crate::models::V1IpBlock>>,
    #[serde(rename = "namespaceSelector", skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<Box<crate::models::V1LabelSelector>>,
    #[serde(rename = "podSelector", skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<Box<crate::models::V1LabelSelector>>,
}

impl V1NetworkPolicyPeer {
    /// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed
    pub fn new() -> V1NetworkPolicyPeer {
        V1NetworkPolicyPeer {
            ip_block: None,
            namespace_selector: None,
            pod_selector: None,
        }
    }
}


