# V1VolumeAttachmentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attach_error** | Option<[**crate::models::V1VolumeError**](v1.VolumeError.md)> |  | [optional]
**attached** | **bool** | attached indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher. | 
**attachment_metadata** | Option<**::std::collections::HashMap<String, String>**> | attachmentMetadata is populated with any information returned by the attach operation, upon successful attach, that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher. | [optional]
**detach_error** | Option<[**crate::models::V1VolumeError**](v1.VolumeError.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


