# V1alpha3DeviceAllocationConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**opaque** | Option<[**crate::models::V1alpha3OpaqueDeviceConfiguration**](v1alpha3.OpaqueDeviceConfiguration.md)> |  | [optional]
**requests** | Option<**Vec<String>**> | Requests lists the names of requests where the configuration applies. If empty, its applies to all requests. | [optional]
**source** | **String** | Source records whether the configuration comes from a class and thus is not something that a normal user would have been able to set or from a claim. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


