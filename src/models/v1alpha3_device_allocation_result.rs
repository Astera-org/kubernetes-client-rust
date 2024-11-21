/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3DeviceAllocationResult : DeviceAllocationResult is the result of allocating devices.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3DeviceAllocationResult {
    /// This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.  This includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters.
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<crate::models::V1alpha3DeviceAllocationConfiguration>>,
    /// Results lists all allocated devices.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::V1alpha3DeviceRequestAllocationResult>>,
}

impl V1alpha3DeviceAllocationResult {
    /// DeviceAllocationResult is the result of allocating devices.
    pub fn new() -> V1alpha3DeviceAllocationResult {
        V1alpha3DeviceAllocationResult {
            config: None,
            results: None,
        }
    }
}


