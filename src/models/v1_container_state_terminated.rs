/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ContainerStateTerminated : ContainerStateTerminated is a terminated state of a container.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ContainerStateTerminated {
    /// Container's ID in the format '<type>://<container_id>'
    #[serde(rename = "containerID", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// Exit status from the last termination of the container
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    /// Time at which the container last terminated
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// Message regarding the last termination of the container
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// (brief) reason from the last termination of the container
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Signal from the last termination of the container
    #[serde(rename = "signal", skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// Time at which previous execution of the container started
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
}

impl V1ContainerStateTerminated {
    /// ContainerStateTerminated is a terminated state of a container.
    pub fn new(exit_code: i32) -> V1ContainerStateTerminated {
        V1ContainerStateTerminated {
            container_id: None,
            exit_code,
            finished_at: None,
            message: None,
            reason: None,
            signal: None,
            started_at: None,
        }
    }
}


