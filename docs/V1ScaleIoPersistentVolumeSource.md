# V1ScaleIoPersistentVolumeSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fs_type** | Option<**String**> | fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Default is \"xfs\" | [optional]
**gateway** | **String** | gateway is the host address of the ScaleIO API Gateway. | 
**protection_domain** | Option<**String**> | protectionDomain is the name of the ScaleIO Protection Domain for the configured storage. | [optional]
**read_only** | Option<**bool**> | readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. | [optional]
**secret_ref** | [**crate::models::V1SecretReference**](v1.SecretReference.md) |  | 
**ssl_enabled** | Option<**bool**> | sslEnabled is the flag to enable/disable SSL communication with Gateway, default false | [optional]
**storage_mode** | Option<**String**> | storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned. | [optional]
**storage_pool** | Option<**String**> | storagePool is the ScaleIO Storage Pool associated with the protection domain. | [optional]
**system** | **String** | system is the name of the storage system as configured in ScaleIO. | 
**volume_name** | Option<**String**> | volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


