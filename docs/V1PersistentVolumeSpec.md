# V1PersistentVolumeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | Option<**Vec<String>**> | accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes | [optional]
**aws_elastic_block_store** | Option<[**crate::models::V1AwsElasticBlockStoreVolumeSource**](v1.AWSElasticBlockStoreVolumeSource.md)> |  | [optional]
**azure_disk** | Option<[**crate::models::V1AzureDiskVolumeSource**](v1.AzureDiskVolumeSource.md)> |  | [optional]
**azure_file** | Option<[**crate::models::V1AzureFilePersistentVolumeSource**](v1.AzureFilePersistentVolumeSource.md)> |  | [optional]
**capacity** | Option<**::std::collections::HashMap<String, String>**> | capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity | [optional]
**cephfs** | Option<[**crate::models::V1CephFsPersistentVolumeSource**](v1.CephFSPersistentVolumeSource.md)> |  | [optional]
**cinder** | Option<[**crate::models::V1CinderPersistentVolumeSource**](v1.CinderPersistentVolumeSource.md)> |  | [optional]
**claim_ref** | Option<[**crate::models::V1ObjectReference**](v1.ObjectReference.md)> |  | [optional]
**csi** | Option<[**crate::models::V1CsiPersistentVolumeSource**](v1.CSIPersistentVolumeSource.md)> |  | [optional]
**fc** | Option<[**crate::models::V1FcVolumeSource**](v1.FCVolumeSource.md)> |  | [optional]
**flex_volume** | Option<[**crate::models::V1FlexPersistentVolumeSource**](v1.FlexPersistentVolumeSource.md)> |  | [optional]
**flocker** | Option<[**crate::models::V1FlockerVolumeSource**](v1.FlockerVolumeSource.md)> |  | [optional]
**gce_persistent_disk** | Option<[**crate::models::V1GcePersistentDiskVolumeSource**](v1.GCEPersistentDiskVolumeSource.md)> |  | [optional]
**glusterfs** | Option<[**crate::models::V1GlusterfsPersistentVolumeSource**](v1.GlusterfsPersistentVolumeSource.md)> |  | [optional]
**host_path** | Option<[**crate::models::V1HostPathVolumeSource**](v1.HostPathVolumeSource.md)> |  | [optional]
**iscsi** | Option<[**crate::models::V1IscsiPersistentVolumeSource**](v1.ISCSIPersistentVolumeSource.md)> |  | [optional]
**local** | Option<[**crate::models::V1LocalVolumeSource**](v1.LocalVolumeSource.md)> |  | [optional]
**mount_options** | Option<**Vec<String>**> | mountOptions is the list of mount options, e.g. [\"ro\", \"soft\"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options | [optional]
**nfs** | Option<[**crate::models::V1NfsVolumeSource**](v1.NFSVolumeSource.md)> |  | [optional]
**node_affinity** | Option<[**crate::models::V1VolumeNodeAffinity**](v1.VolumeNodeAffinity.md)> |  | [optional]
**persistent_volume_reclaim_policy** | Option<**String**> | persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming | [optional]
**photon_persistent_disk** | Option<[**crate::models::V1PhotonPersistentDiskVolumeSource**](v1.PhotonPersistentDiskVolumeSource.md)> |  | [optional]
**portworx_volume** | Option<[**crate::models::V1PortworxVolumeSource**](v1.PortworxVolumeSource.md)> |  | [optional]
**quobyte** | Option<[**crate::models::V1QuobyteVolumeSource**](v1.QuobyteVolumeSource.md)> |  | [optional]
**rbd** | Option<[**crate::models::V1RbdPersistentVolumeSource**](v1.RBDPersistentVolumeSource.md)> |  | [optional]
**scale_io** | Option<[**crate::models::V1ScaleIoPersistentVolumeSource**](v1.ScaleIOPersistentVolumeSource.md)> |  | [optional]
**storage_class_name** | Option<**String**> | storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass. | [optional]
**storageos** | Option<[**crate::models::V1StorageOsPersistentVolumeSource**](v1.StorageOSPersistentVolumeSource.md)> |  | [optional]
**volume_attributes_class_name** | Option<**String**> | Name of VolumeAttributesClass to which this persistent volume belongs. Empty value is not allowed. When this field is not set, it indicates that this volume does not belong to any VolumeAttributesClass. This field is mutable and can be changed by the CSI driver after a volume has been updated successfully to a new class. For an unbound PersistentVolume, the volumeAttributesClassName will be matched with unbound PersistentVolumeClaims during the binding process. This is a beta field and requires enabling VolumeAttributesClass feature (off by default). | [optional]
**volume_mode** | Option<**String**> | volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec. | [optional]
**vsphere_volume** | Option<[**crate::models::V1VsphereVirtualDiskVolumeSource**](v1.VsphereVirtualDiskVolumeSource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


