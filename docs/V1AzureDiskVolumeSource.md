# V1AzureDiskVolumeSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caching_mode** | Option<**String**> | cachingMode is the Host Caching mode: None, Read Only, Read Write. | [optional]
**disk_name** | **String** | diskName is the Name of the data disk in the blob storage | 
**disk_uri** | **String** | diskURI is the URI of data disk in the blob storage | 
**fs_type** | Option<**String**> | fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. | [optional]
**kind** | Option<**String**> | kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared | [optional]
**read_only** | Option<**bool**> | readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


