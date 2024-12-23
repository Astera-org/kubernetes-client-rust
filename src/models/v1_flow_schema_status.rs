/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1FlowSchemaStatus : FlowSchemaStatus represents the current state of a FlowSchema.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1FlowSchemaStatus {
    /// `conditions` is a list of the current states of FlowSchema.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1FlowSchemaCondition>>,
}

impl V1FlowSchemaStatus {
    /// FlowSchemaStatus represents the current state of a FlowSchema.
    pub fn new() -> V1FlowSchemaStatus {
        V1FlowSchemaStatus {
            conditions: None,
        }
    }
}


