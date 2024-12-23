# V1StatefulSetSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | Option<**i32**> | Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) | [optional]
**ordinals** | Option<[**crate::models::V1StatefulSetOrdinals**](v1.StatefulSetOrdinals.md)> |  | [optional]
**persistent_volume_claim_retention_policy** | Option<[**crate::models::V1StatefulSetPersistentVolumeClaimRetentionPolicy**](v1.StatefulSetPersistentVolumeClaimRetentionPolicy.md)> |  | [optional]
**pod_management_policy** | Option<**String**> | podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once. | [optional]
**replicas** | Option<**i32**> | replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1. | [optional]
**revision_history_limit** | Option<**i32**> | revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10. | [optional]
**selector** | [**crate::models::V1LabelSelector**](v1.LabelSelector.md) |  | 
**service_name** | **String** | serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where \"pod-specific-string\" is managed by the StatefulSet controller. | 
**template** | [**crate::models::V1PodTemplateSpec**](v1.PodTemplateSpec.md) |  | 
**update_strategy** | Option<[**crate::models::V1StatefulSetUpdateStrategy**](v1.StatefulSetUpdateStrategy.md)> |  | [optional]
**volume_claim_templates** | Option<[**Vec<crate::models::V1PersistentVolumeClaim>**](v1.PersistentVolumeClaim.md)> | volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


