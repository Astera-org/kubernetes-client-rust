# V1DeploymentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_replicas** | Option<**i32**> | Total number of available pods (ready for at least minReadySeconds) targeted by this deployment. | [optional]
**collision_count** | Option<**i32**> | Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet. | [optional]
**conditions** | Option<[**Vec<crate::models::V1DeploymentCondition>**](v1.DeploymentCondition.md)> | Represents the latest available observations of a deployment's current state. | [optional]
**observed_generation** | Option<**i64**> | The generation observed by the deployment controller. | [optional]
**ready_replicas** | Option<**i32**> | readyReplicas is the number of pods targeted by this Deployment with a Ready Condition. | [optional]
**replicas** | Option<**i32**> | Total number of non-terminated pods targeted by this deployment (their labels match the selector). | [optional]
**unavailable_replicas** | Option<**i32**> | Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created. | [optional]
**updated_replicas** | Option<**i32**> | Total number of non-terminated pods targeted by this deployment that have the desired template spec. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


