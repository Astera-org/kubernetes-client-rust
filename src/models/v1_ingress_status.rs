/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IngressStatus : IngressStatus describe the current state of the Ingress.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1IngressStatus {
    #[serde(rename = "loadBalancer", skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<Box<crate::models::V1IngressLoadBalancerStatus>>,
}

impl V1IngressStatus {
    /// IngressStatus describe the current state of the Ingress.
    pub fn new() -> V1IngressStatus {
        V1IngressStatus {
            load_balancer: None,
        }
    }
}


