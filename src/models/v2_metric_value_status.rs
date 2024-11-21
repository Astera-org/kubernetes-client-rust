/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2MetricValueStatus : MetricValueStatus holds the current value for a metric



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V2MetricValueStatus {
    /// currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
    #[serde(rename = "averageUtilization", skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
    /// averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    #[serde(rename = "averageValue", skip_serializing_if = "Option::is_none")]
    pub average_value: Option<String>,
    /// value is the current value of the metric (as a quantity).
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl V2MetricValueStatus {
    /// MetricValueStatus holds the current value for a metric
    pub fn new() -> V2MetricValueStatus {
        V2MetricValueStatus {
            average_utilization: None,
            average_value: None,
            value: None,
        }
    }
}


