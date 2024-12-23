/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1StatusDetails : StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1StatusDetails {
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    #[serde(rename = "causes", skip_serializing_if = "Option::is_none")]
    pub causes: Option<Vec<crate::models::V1StatusCause>>,
    /// The group attribute of the resource associated with the status StatusReason.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.
    #[serde(rename = "retryAfterSeconds", skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i32>,
    /// UID of the resource. (when there is a single resource which can be described). More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl V1StatusDetails {
    /// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
    pub fn new() -> V1StatusDetails {
        V1StatusDetails {
            causes: None,
            group: None,
            kind: None,
            name: None,
            retry_after_seconds: None,
            uid: None,
        }
    }
}


