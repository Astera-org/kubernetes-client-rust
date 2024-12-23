/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Probe : Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1Probe {
    #[serde(rename = "exec", skip_serializing_if = "Option::is_none")]
    pub exec: Option<Box<crate::models::V1ExecAction>>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold", skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "grpc", skip_serializing_if = "Option::is_none")]
    pub grpc: Option<Box<crate::models::V1GrpcAction>>,
    #[serde(rename = "httpGet", skip_serializing_if = "Option::is_none")]
    pub http_get: Option<Box<crate::models::V1HttpGetAction>>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename = "initialDelaySeconds", skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds", skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(rename = "successThreshold", skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(rename = "tcpSocket", skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<Box<crate::models::V1TcpSocketAction>>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(rename = "terminationGracePeriodSeconds", skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

impl V1Probe {
    /// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
    pub fn new() -> V1Probe {
        V1Probe {
            exec: None,
            failure_threshold: None,
            grpc: None,
            http_get: None,
            initial_delay_seconds: None,
            period_seconds: None,
            success_threshold: None,
            tcp_socket: None,
            termination_grace_period_seconds: None,
            timeout_seconds: None,
        }
    }
}


