/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1TcpSocketAction : TCPSocketAction describes an action based on opening a socket



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1TcpSocketAction {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    #[serde(rename = "port")]
    pub port: serde_json::Value,
}

impl V1TcpSocketAction {
    /// TCPSocketAction describes an action based on opening a socket
    pub fn new(port: serde_json::Value) -> V1TcpSocketAction {
        V1TcpSocketAction {
            host: None,
            port,
        }
    }
}


