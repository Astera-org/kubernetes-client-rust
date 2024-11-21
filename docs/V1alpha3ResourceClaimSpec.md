# V1alpha3ResourceClaimSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**controller** | Option<**String**> | Controller is the name of the DRA driver that is meant to handle allocation of this claim. If empty, allocation is handled by the scheduler while scheduling a pod.  Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.  This is an alpha field and requires enabling the DRAControlPlaneController feature gate. | [optional]
**devices** | Option<[**crate::models::V1alpha3DeviceClaim**](v1alpha3.DeviceClaim.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


