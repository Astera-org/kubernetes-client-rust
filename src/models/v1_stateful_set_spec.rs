/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1StatefulSetSpec : A StatefulSetSpec is the specification of a StatefulSet.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1StatefulSetSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(rename = "minReadySeconds", skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    #[serde(rename = "ordinals", skip_serializing_if = "Option::is_none")]
    pub ordinals: Option<Box<crate::models::V1StatefulSetOrdinals>>,
    #[serde(rename = "persistentVolumeClaimRetentionPolicy", skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_retention_policy: Option<Box<crate::models::V1StatefulSetPersistentVolumeClaimRetentionPolicy>>,
    /// podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
    #[serde(rename = "podManagementPolicy", skip_serializing_if = "Option::is_none")]
    pub pod_management_policy: Option<String>,
    /// replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.
    #[serde(rename = "revisionHistoryLimit", skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    #[serde(rename = "selector")]
    pub selector: Box<crate::models::V1LabelSelector>,
    /// serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where \"pod-specific-string\" is managed by the StatefulSet controller.
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "template")]
    pub template: Box<crate::models::V1PodTemplateSpec>,
    #[serde(rename = "updateStrategy", skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<Box<crate::models::V1StatefulSetUpdateStrategy>>,
    /// volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
    #[serde(rename = "volumeClaimTemplates", skip_serializing_if = "Option::is_none")]
    pub volume_claim_templates: Option<Vec<crate::models::V1PersistentVolumeClaim>>,
}

impl V1StatefulSetSpec {
    /// A StatefulSetSpec is the specification of a StatefulSet.
    pub fn new(selector: crate::models::V1LabelSelector, service_name: String, template: crate::models::V1PodTemplateSpec) -> V1StatefulSetSpec {
        V1StatefulSetSpec {
            min_ready_seconds: None,
            ordinals: None,
            persistent_volume_claim_retention_policy: None,
            pod_management_policy: None,
            replicas: None,
            revision_history_limit: None,
            selector: Box::new(selector),
            service_name,
            template: Box::new(template),
            update_strategy: None,
            volume_claim_templates: None,
        }
    }
}

