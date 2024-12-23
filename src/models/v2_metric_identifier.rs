/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2MetricIdentifier : MetricIdentifier defines the name and optionally selector for a metric



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V2MetricIdentifier {
    /// name is the name of the given metric
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::models::V1LabelSelector>>,
}

impl V2MetricIdentifier {
    /// MetricIdentifier defines the name and optionally selector for a metric
    pub fn new(name: String) -> V2MetricIdentifier {
        V2MetricIdentifier {
            name,
            selector: None,
        }
    }
}


