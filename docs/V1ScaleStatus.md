# V1ScaleStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**replicas** | **i32** | replicas is the actual number of observed instances of the scaled object. | 
**selector** | Option<**String**> | selector is the label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


