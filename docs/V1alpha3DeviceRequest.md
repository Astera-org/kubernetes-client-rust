# V1alpha3DeviceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_access** | Option<**bool**> | AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations. | [optional]
**allocation_mode** | Option<**String**> | AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:  - ExactCount: This request is for a specific number of devices.   This is the default. The exact number is provided in the   count field.  - All: This request is for all of the matching devices in a pool.   Allocation will fail if some devices are already allocated,   unless adminAccess is requested.  If AlloctionMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.  More modes may get added in the future. Clients must refuse to handle requests with unknown modes. | [optional]
**count** | Option<**i64**> | Count is used only when the count mode is \"ExactCount\". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one. | [optional]
**device_class_name** | **String** | DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.  A class is required. Which classes are available depends on the cluster.  Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference. | 
**name** | **String** | Name can be used to reference this request in a pod.spec.containers[].resources.claims entry and in a constraint of the claim.  Must be a DNS label. | 
**selectors** | Option<[**Vec<crate::models::V1alpha3DeviceSelector>**](v1alpha3.DeviceSelector.md)> | Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

