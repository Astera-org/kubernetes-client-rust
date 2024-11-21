/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2PodsMetricStatus : PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V2PodsMetricStatus {
    #[serde(rename = "current")]
    pub current: Box<crate::models::V2MetricValueStatus>,
    #[serde(rename = "metric")]
    pub metric: Box<crate::models::V2MetricIdentifier>,
}

impl V2PodsMetricStatus {
    /// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
    pub fn new(current: crate::models::V2MetricValueStatus, metric: crate::models::V2MetricIdentifier) -> V2PodsMetricStatus {
        V2PodsMetricStatus {
            current: Box::new(current),
            metric: Box::new(metric),
        }
    }
}


