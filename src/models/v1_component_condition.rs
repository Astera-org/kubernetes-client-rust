/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ComponentCondition : Information about the condition of a component.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ComponentCondition {
    /// Condition error code for a component. For example, a health check error code.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Message about the condition for a component. For example, information about a health check.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status of the condition for a component. Valid values for \"Healthy\": \"True\", \"False\", or \"Unknown\".
    #[serde(rename = "status")]
    pub status: String,
    /// Type of condition for a component. Valid value: \"Healthy\"
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1ComponentCondition {
    /// Information about the condition of a component.
    pub fn new(status: String, _type: String) -> V1ComponentCondition {
        V1ComponentCondition {
            error: None,
            message: None,
            status,
            _type,
        }
    }
}


