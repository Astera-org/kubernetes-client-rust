# V1StorageClass

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_volume_expansion** | Option<**bool**> | allowVolumeExpansion shows whether the storage class allow volume expand. | [optional]
**allowed_topologies** | Option<[**Vec<crate::models::V1TopologySelectorTerm>**](v1.TopologySelectorTerm.md)> | allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature. | [optional]
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | Option<[**crate::models::V1ObjectMeta**](v1.ObjectMeta.md)> |  | [optional]
**mount_options** | Option<**Vec<String>**> | mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class. e.g. [\"ro\", \"soft\"]. Not validated - mount of the PVs will simply fail if one is invalid. | [optional]
**parameters** | Option<**::std::collections::HashMap<String, String>**> | parameters holds the parameters for the provisioner that should create volumes of this storage class. | [optional]
**provisioner** | **String** | provisioner indicates the type of the provisioner. | 
**reclaim_policy** | Option<**String**> | reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes of this storage class. Defaults to Delete. | [optional]
**volume_binding_mode** | Option<**String**> | volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


