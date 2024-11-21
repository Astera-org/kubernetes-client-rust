# V1SuccessPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rules** | [**Vec<crate::models::V1SuccessPolicyRule>**](v1.SuccessPolicyRule.md) | rules represents the list of alternative rules for the declaring the Jobs as successful before `.status.succeeded >= .spec.completions`. Once any of the rules are met, the \"SucceededCriteriaMet\" condition is added, and the lingering pods are removed. The terminal state for such a Job has the \"Complete\" condition. Additionally, these rules are evaluated in order; Once the Job meets one of the rules, other rules are ignored. At most 20 elements are allowed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


