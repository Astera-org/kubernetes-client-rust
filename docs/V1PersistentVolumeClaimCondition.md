# V1PersistentVolumeClaimCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | Option<**String**> | lastProbeTime is the time we probed the condition. | [optional]
**last_transition_time** | Option<**String**> | lastTransitionTime is the time the condition transitioned from one status to another. | [optional]
**message** | Option<**String**> | message is the human-readable message indicating details about last transition. | [optional]
**reason** | Option<**String**> | reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"Resizing\" that means the underlying persistent volume is being resized. | [optional]
**status** | **String** |  | 
**_type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


