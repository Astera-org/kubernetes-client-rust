# V1alpha3DeviceConstraint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**match_attribute** | Option<**String**> | MatchAttribute requires that all devices in question have this attribute and that its type and value are the same across those devices.  For example, if you specified \"dra.example.com/numa\" (a hypothetical example!), then only devices in the same NUMA node will be chosen. A device which does not have that attribute will not be chosen. All devices should use a value of the same type for this attribute because that is part of its specification, but if one device doesn't, then it also will not be chosen.  Must include the domain qualifier. | [optional]
**requests** | Option<**Vec<String>**> | Requests is a list of the one or more requests in this claim which must co-satisfy this constraint. If a request is fulfilled by multiple devices, then all of the devices must satisfy the constraint. If this is not specified, this constraint applies to all requests in this claim. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


