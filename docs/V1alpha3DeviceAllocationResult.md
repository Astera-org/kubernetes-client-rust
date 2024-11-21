# V1alpha3DeviceAllocationResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**Vec<crate::models::V1alpha3DeviceAllocationConfiguration>**](v1alpha3.DeviceAllocationConfiguration.md)> | This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.  This includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters. | [optional]
**results** | Option<[**Vec<crate::models::V1alpha3DeviceRequestAllocationResult>**](v1alpha3.DeviceRequestAllocationResult.md)> | Results lists all allocated devices. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


