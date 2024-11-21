/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ContainerPort : ContainerPort represents a network port in a single container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ContainerPort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
    #[serde(rename = "containerPort")]
    pub container_port: i32,
    /// What host IP to bind the external port to.
    #[serde(rename = "hostIP", skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[serde(rename = "hostPort", skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\".
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl V1ContainerPort {
    /// ContainerPort represents a network port in a single container.
    pub fn new(container_port: i32) -> V1ContainerPort {
        V1ContainerPort {
            container_port,
            host_ip: None,
            host_port: None,
            name: None,
            protocol: None,
        }
    }
}


