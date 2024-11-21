/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Volume : Volume represents a named volume in a pod that may be accessed by any container in the pod.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1Volume {
    #[serde(rename = "awsElasticBlockStore", skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<Box<crate::models::V1AwsElasticBlockStoreVolumeSource>>,
    #[serde(rename = "azureDisk", skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<Box<crate::models::V1AzureDiskVolumeSource>>,
    #[serde(rename = "azureFile", skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<Box<crate::models::V1AzureFileVolumeSource>>,
    #[serde(rename = "cephfs", skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<Box<crate::models::V1CephFsVolumeSource>>,
    #[serde(rename = "cinder", skip_serializing_if = "Option::is_none")]
    pub cinder: Option<Box<crate::models::V1CinderVolumeSource>>,
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<Box<crate::models::V1ConfigMapVolumeSource>>,
    #[serde(rename = "csi", skip_serializing_if = "Option::is_none")]
    pub csi: Option<Box<crate::models::V1CsiVolumeSource>>,
    #[serde(rename = "downwardAPI", skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<Box<crate::models::V1DownwardApiVolumeSource>>,
    #[serde(rename = "emptyDir", skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<Box<crate::models::V1EmptyDirVolumeSource>>,
    #[serde(rename = "ephemeral", skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<Box<crate::models::V1EphemeralVolumeSource>>,
    #[serde(rename = "fc", skip_serializing_if = "Option::is_none")]
    pub fc: Option<Box<crate::models::V1FcVolumeSource>>,
    #[serde(rename = "flexVolume", skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<Box<crate::models::V1FlexVolumeSource>>,
    #[serde(rename = "flocker", skip_serializing_if = "Option::is_none")]
    pub flocker: Option<Box<crate::models::V1FlockerVolumeSource>>,
    #[serde(rename = "gcePersistentDisk", skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<Box<crate::models::V1GcePersistentDiskVolumeSource>>,
    #[serde(rename = "gitRepo", skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<Box<crate::models::V1GitRepoVolumeSource>>,
    #[serde(rename = "glusterfs", skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<Box<crate::models::V1GlusterfsVolumeSource>>,
    #[serde(rename = "hostPath", skip_serializing_if = "Option::is_none")]
    pub host_path: Option<Box<crate::models::V1HostPathVolumeSource>>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<crate::models::V1ImageVolumeSource>>,
    #[serde(rename = "iscsi", skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<Box<crate::models::V1IscsiVolumeSource>>,
    /// name of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nfs", skip_serializing_if = "Option::is_none")]
    pub nfs: Option<Box<crate::models::V1NfsVolumeSource>>,
    #[serde(rename = "persistentVolumeClaim", skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<Box<crate::models::V1PersistentVolumeClaimVolumeSource>>,
    #[serde(rename = "photonPersistentDisk", skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<Box<crate::models::V1PhotonPersistentDiskVolumeSource>>,
    #[serde(rename = "portworxVolume", skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<Box<crate::models::V1PortworxVolumeSource>>,
    #[serde(rename = "projected", skip_serializing_if = "Option::is_none")]
    pub projected: Option<Box<crate::models::V1ProjectedVolumeSource>>,
    #[serde(rename = "quobyte", skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<Box<crate::models::V1QuobyteVolumeSource>>,
    #[serde(rename = "rbd", skip_serializing_if = "Option::is_none")]
    pub rbd: Option<Box<crate::models::V1RbdVolumeSource>>,
    #[serde(rename = "scaleIO", skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<Box<crate::models::V1ScaleIoVolumeSource>>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<Box<crate::models::V1SecretVolumeSource>>,
    #[serde(rename = "storageos", skip_serializing_if = "Option::is_none")]
    pub storageos: Option<Box<crate::models::V1StorageOsVolumeSource>>,
    #[serde(rename = "vsphereVolume", skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<Box<crate::models::V1VsphereVirtualDiskVolumeSource>>,
}

impl V1Volume {
    /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
    pub fn new(name: String) -> V1Volume {
        V1Volume {
            aws_elastic_block_store: None,
            azure_disk: None,
            azure_file: None,
            cephfs: None,
            cinder: None,
            config_map: None,
            csi: None,
            downward_api: None,
            empty_dir: None,
            ephemeral: None,
            fc: None,
            flex_volume: None,
            flocker: None,
            gce_persistent_disk: None,
            git_repo: None,
            glusterfs: None,
            host_path: None,
            image: None,
            iscsi: None,
            name,
            nfs: None,
            persistent_volume_claim: None,
            photon_persistent_disk: None,
            portworx_volume: None,
            projected: None,
            quobyte: None,
            rbd: None,
            scale_io: None,
            secret: None,
            storageos: None,
            vsphere_volume: None,
        }
    }
}

