/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2HorizontalPodAutoscalerCondition : HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V2HorizontalPodAutoscalerCondition {
    /// lastTransitionTime is the last time the condition transitioned from one status to another
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// message is a human-readable explanation containing details about the transition
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// reason is the reason for the condition's last transition.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// status is the status of the condition (True, False, Unknown)
    #[serde(rename = "status")]
    pub status: String,
    /// type describes the current condition
    #[serde(rename = "type")]
    pub _type: String,
}

impl V2HorizontalPodAutoscalerCondition {
    /// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
    pub fn new(status: String, _type: String) -> V2HorizontalPodAutoscalerCondition {
        V2HorizontalPodAutoscalerCondition {
            last_transition_time: None,
            message: None,
            reason: None,
            status,
            _type,
        }
    }
}


