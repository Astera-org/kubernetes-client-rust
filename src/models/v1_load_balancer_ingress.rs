/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1LoadBalancerIngress : LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LoadBalancerIngress {
    /// Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified. Setting this to \"VIP\" indicates that traffic is delivered to the node with the destination set to the load-balancer's IP and port. Setting this to \"Proxy\" indicates that traffic is delivered to the node or pod with the destination set to the node's IP and node port or the pod's IP and port. Service implementations may use this information to adjust traffic routing.
    #[serde(rename = "ipMode", skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<String>,
    /// Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<crate::models::V1PortStatus>>,
}

impl V1LoadBalancerIngress {
    /// LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.
    pub fn new() -> V1LoadBalancerIngress {
        V1LoadBalancerIngress {
            hostname: None,
            ip: None,
            ip_mode: None,
            ports: None,
        }
    }
}


