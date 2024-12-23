# V1CephFsPersistentVolumeSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monitors** | **Vec<String>** | monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it | 
**path** | Option<**String**> | path is Optional: Used as the mounted root, rather than the full Ceph tree, default is / | [optional]
**read_only** | Option<**bool**> | readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it | [optional]
**secret_file** | Option<**String**> | secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it | [optional]
**secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**user** | Option<**String**> | user is Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


