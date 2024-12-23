/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1SessionAffinityConfig : SessionAffinityConfig represents the configurations of session affinity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1SessionAffinityConfig {
    #[serde(rename = "clientIP", skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<Box<crate::models::V1ClientIpConfig>>,
}

impl V1SessionAffinityConfig {
    /// SessionAffinityConfig represents the configurations of session affinity.
    pub fn new() -> V1SessionAffinityConfig {
        V1SessionAffinityConfig {
            client_ip: None,
        }
    }
}


