# V1alpha3DeviceClassSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**Vec<crate::models::V1alpha3DeviceClassConfiguration>**](v1alpha3.DeviceClassConfiguration.md)> | Config defines configuration parameters that apply to each device that is claimed via this class. Some classses may potentially be satisfied by multiple drivers, so each instance of a vendor configuration applies to exactly one driver.  They are passed to the driver, but are not considered while allocating the claim. | [optional]
**selectors** | Option<[**Vec<crate::models::V1alpha3DeviceSelector>**](v1alpha3.DeviceSelector.md)> | Each selector must be satisfied by a device which is claimed via this class. | [optional]
**suitable_nodes** | Option<[**crate::models::V1NodeSelector**](v1.NodeSelector.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


