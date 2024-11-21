/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3DeviceRequestAllocationResult : DeviceRequestAllocationResult contains the allocation result for one request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3DeviceRequestAllocationResult {
    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    #[serde(rename = "device")]
    pub device: String,
    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.  Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    #[serde(rename = "driver")]
    pub driver: String,
    /// This name together with the driver name and the device name field identify which device was allocated (`<driver name>/<pool name>/<device name>`).  Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    #[serde(rename = "pool")]
    pub pool: String,
    /// Request is the name of the request in the claim which caused this device to be allocated. Multiple devices may have been allocated per request.
    #[serde(rename = "request")]
    pub request: String,
}

impl V1alpha3DeviceRequestAllocationResult {
    /// DeviceRequestAllocationResult contains the allocation result for one request.
    pub fn new(device: String, driver: String, pool: String, request: String) -> V1alpha3DeviceRequestAllocationResult {
        V1alpha3DeviceRequestAllocationResult {
            device,
            driver,
            pool,
            request,
        }
    }
}


