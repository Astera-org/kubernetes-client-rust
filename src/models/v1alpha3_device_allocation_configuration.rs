/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3DeviceAllocationConfiguration : DeviceAllocationConfiguration gets embedded in an AllocationResult.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3DeviceAllocationConfiguration {
    #[serde(rename = "opaque", skip_serializing_if = "Option::is_none")]
    pub opaque: Option<Box<crate::models::V1alpha3OpaqueDeviceConfiguration>>,
    /// Requests lists the names of requests where the configuration applies. If empty, its applies to all requests.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<String>>,
    /// Source records whether the configuration comes from a class and thus is not something that a normal user would have been able to set or from a claim.
    #[serde(rename = "source")]
    pub source: String,
}

impl V1alpha3DeviceAllocationConfiguration {
    /// DeviceAllocationConfiguration gets embedded in an AllocationResult.
    pub fn new(source: String) -> V1alpha3DeviceAllocationConfiguration {
        V1alpha3DeviceAllocationConfiguration {
            opaque: None,
            requests: None,
            source,
        }
    }
}

