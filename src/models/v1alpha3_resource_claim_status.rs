/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3ResourceClaimStatus : ResourceClaimStatus tracks whether the resource has been allocated and what the result of that was.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3ResourceClaimStatus {
    #[serde(rename = "allocation", skip_serializing_if = "Option::is_none")]
    pub allocation: Option<Box<crate::models::V1alpha3AllocationResult>>,
    /// Indicates that a claim is to be deallocated. While this is set, no new consumers may be added to ReservedFor.  This is only used if the claim needs to be deallocated by a DRA driver. That driver then must deallocate this claim and reset the field together with clearing the Allocation field.  This is an alpha field and requires enabling the DRAControlPlaneController feature gate.
    #[serde(rename = "deallocationRequested", skip_serializing_if = "Option::is_none")]
    pub deallocation_requested: Option<bool>,
    /// ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started. A claim that is in use or might be in use because it has been reserved must not get deallocated.  In a cluster with multiple scheduler instances, two pods might get scheduled concurrently by different schedulers. When they reference the same ResourceClaim which already has reached its maximum number of consumers, only one pod can be scheduled.  Both schedulers try to add their pod to the claim.status.reservedFor field, but only the update that reaches the API server first gets stored. The other one fails with an error and the scheduler which issued it knows that it must put the pod back into the queue, waiting for the ResourceClaim to become usable again.  There can be at most 32 such reservations. This may get increased in the future, but not reduced.
    #[serde(rename = "reservedFor", skip_serializing_if = "Option::is_none")]
    pub reserved_for: Option<Vec<crate::models::V1alpha3ResourceClaimConsumerReference>>,
}

impl V1alpha3ResourceClaimStatus {
    /// ResourceClaimStatus tracks whether the resource has been allocated and what the result of that was.
    pub fn new() -> V1alpha3ResourceClaimStatus {
        V1alpha3ResourceClaimStatus {
            allocation: None,
            deallocation_requested: None,
            reserved_for: None,
        }
    }
}


