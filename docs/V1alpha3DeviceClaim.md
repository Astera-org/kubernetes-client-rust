# V1alpha3DeviceClaim

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**Vec<crate::models::V1alpha3DeviceClaimConfiguration>**](v1alpha3.DeviceClaimConfiguration.md)> | This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim. | [optional]
**constraints** | Option<[**Vec<crate::models::V1alpha3DeviceConstraint>**](v1alpha3.DeviceConstraint.md)> | These constraints must be satisfied by the set of devices that get allocated for the claim. | [optional]
**requests** | Option<[**Vec<crate::models::V1alpha3DeviceRequest>**](v1alpha3.DeviceRequest.md)> | Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


