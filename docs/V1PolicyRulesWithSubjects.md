# V1PolicyRulesWithSubjects

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**non_resource_rules** | Option<[**Vec<crate::models::V1NonResourcePolicyRule>**](v1.NonResourcePolicyRule.md)> | `nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL. | [optional]
**resource_rules** | Option<[**Vec<crate::models::V1ResourcePolicyRule>**](v1.ResourcePolicyRule.md)> | `resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty. | [optional]
**subjects** | [**Vec<crate::models::FlowcontrolV1Subject>**](flowcontrol.v1.Subject.md) | subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


