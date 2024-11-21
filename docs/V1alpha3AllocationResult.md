# V1alpha3AllocationResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**controller** | Option<**String**> | Controller is the name of the DRA driver which handled the allocation. That driver is also responsible for deallocating the claim. It is empty when the claim can be deallocated without involving a driver.  A driver may allocate devices provided by other drivers, so this driver name here can be different from the driver names listed for the results.  This is an alpha field and requires enabling the DRAControlPlaneController feature gate. | [optional]
**devices** | Option<[**crate::models::V1alpha3DeviceAllocationResult**](v1alpha3.DeviceAllocationResult.md)> |  | [optional]
**node_selector** | Option<[**crate::models::V1NodeSelector**](v1.NodeSelector.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


