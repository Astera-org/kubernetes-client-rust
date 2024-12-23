/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NetworkPolicyPort : NetworkPolicyPort describes a port to allow traffic on



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NetworkPolicyPort {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
    #[serde(rename = "endPort", skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
    /// port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<serde_json::Value>,
    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl V1NetworkPolicyPort {
    /// NetworkPolicyPort describes a port to allow traffic on
    pub fn new() -> V1NetworkPolicyPort {
        V1NetworkPolicyPort {
            end_port: None,
            port: None,
            protocol: None,
        }
    }
}


