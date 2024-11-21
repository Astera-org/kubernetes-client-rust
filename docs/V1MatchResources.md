# V1MatchResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exclude_resource_rules** | Option<[**Vec<crate::models::V1NamedRuleWithOperations>**](v1.NamedRuleWithOperations.md)> | ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about. The exclude rules take precedence over include rules (if a resource matches both, it is excluded) | [optional]
**match_policy** | Option<**String**> | matchPolicy defines how the \"MatchResources\" list is used to match incoming requests. Allowed values are \"Exact\" or \"Equivalent\".  - Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the ValidatingAdmissionPolicy.  - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the ValidatingAdmissionPolicy.  Defaults to \"Equivalent\" | [optional]
**namespace_selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]
**object_selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]
**resource_rules** | Option<[**Vec<crate::models::V1NamedRuleWithOperations>**](v1.NamedRuleWithOperations.md)> | ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches. The policy cares about an operation if it matches _any_ Rule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


