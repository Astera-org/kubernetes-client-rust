/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1LifecycleHandler : LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LifecycleHandler {
    #[serde(rename = "exec", skip_serializing_if = "Option::is_none")]
    pub exec: Option<Box<crate::models::V1ExecAction>>,
    #[serde(rename = "httpGet", skip_serializing_if = "Option::is_none")]
    pub http_get: Option<Box<crate::models::V1HttpGetAction>>,
    #[serde(rename = "sleep", skip_serializing_if = "Option::is_none")]
    pub sleep: Option<Box<crate::models::V1SleepAction>>,
    #[serde(rename = "tcpSocket", skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<Box<crate::models::V1TcpSocketAction>>,
}

impl V1LifecycleHandler {
    /// LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.
    pub fn new() -> V1LifecycleHandler {
        V1LifecycleHandler {
            exec: None,
            http_get: None,
            sleep: None,
            tcp_socket: None,
        }
    }
}


