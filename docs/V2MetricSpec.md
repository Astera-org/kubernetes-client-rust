# V2MetricSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_resource** | Option<[**crate::models::V2ContainerResourceMetricSource**](v2.ContainerResourceMetricSource.md)> |  | [optional]
**external** | Option<[**crate::models::V2ExternalMetricSource**](v2.ExternalMetricSource.md)> |  | [optional]
**object** | Option<[**crate::models::V2ObjectMetricSource**](v2.ObjectMetricSource.md)> |  | [optional]
**pods** | Option<[**crate::models::V2PodsMetricSource**](v2.PodsMetricSource.md)> |  | [optional]
**resource** | Option<[**crate::models::V2ResourceMetricSource**](v2.ResourceMetricSource.md)> |  | [optional]
**_type** | **String** | type is the type of metric source.  It should be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each mapping to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


