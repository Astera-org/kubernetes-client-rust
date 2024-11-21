# V1CsiPersistentVolumeSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**controller_expand_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**controller_publish_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**driver** | **String** | driver is the name of the driver to use for this volume. Required. | 
**fs_type** | Option<**String**> | fsType to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". | [optional]
**node_expand_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**node_publish_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**node_stage_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**read_only** | Option<**bool**> | readOnly value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write). | [optional]
**volume_attributes** | Option<**::std::collections::HashMap<String, String>**> | volumeAttributes of the volume to publish. | [optional]
**volume_handle** | **String** | volumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


