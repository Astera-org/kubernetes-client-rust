# V2HorizontalPodAutoscalerStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | Option<[**Vec<crate::models::V2HorizontalPodAutoscalerCondition>**](v2.HorizontalPodAutoscalerCondition.md)> | conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met. | [optional]
**current_metrics** | Option<[**Vec<crate::models::V2MetricStatus>**](v2.MetricStatus.md)> | currentMetrics is the last read state of the metrics used by this autoscaler. | [optional]
**current_replicas** | Option<**i32**> | currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler. | [optional]
**desired_replicas** | **i32** | desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. | 
**last_scale_time** | Option<**String**> | lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed. | [optional]
**observed_generation** | Option<**i64**> | observedGeneration is the most recent generation observed by this autoscaler. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


