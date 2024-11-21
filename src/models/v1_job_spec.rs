/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1JobSpec : JobSpec describes how the job execution will look like.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1JobSpec {
    /// Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.
    #[serde(rename = "activeDeadlineSeconds", skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// Specifies the number of retries before marking this job failed. Defaults to 6
    #[serde(rename = "backoffLimit", skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,
    /// Specifies the limit for the number of retries within an index before marking this index as failed. When enabled the number of failures per index is kept in the pod's batch.kubernetes.io/job-index-failure-count annotation. It can only be set when Job's completionMode=Indexed, and the Pod's restart policy is Never. The field is immutable. This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).
    #[serde(rename = "backoffLimitPerIndex", skip_serializing_if = "Option::is_none")]
    pub backoff_limit_per_index: Option<i32>,
    /// completionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.  `NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.  `Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.  More completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, which is possible during upgrades due to version skew, the controller skips updates for the Job.
    #[serde(rename = "completionMode", skip_serializing_if = "Option::is_none")]
    pub completion_mode: Option<String>,
    /// Specifies the desired number of successfully finished pods the job should be run with.  Setting to null means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(rename = "completions", skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    /// ManagedBy field indicates the controller that manages a Job. The k8s Job controller reconciles jobs which don't have this field at all or the field value is the reserved string `kubernetes.io/job-controller`, but skips reconciling Jobs with a custom value for this field. The value must be a valid domain-prefixed path (e.g. acme.io/foo) - all characters before the first \"/\" must be a valid subdomain as defined by RFC 1123. All characters trailing the first \"/\" must be valid HTTP Path characters as defined by RFC 3986. The value cannot exceed 63 characters. This field is immutable.  This field is alpha-level. The job controller accepts setting the field when the feature gate JobManagedBy is enabled (disabled by default).
    #[serde(rename = "managedBy", skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    /// manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    #[serde(rename = "manualSelector", skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,
    /// Specifies the maximal number of failed indexes before marking the Job as failed, when backoffLimitPerIndex is set. Once the number of failed indexes exceeds this number the entire Job is marked as Failed and its execution is terminated. When left as null the job continues execution of all of its indexes and is marked with the `Complete` Job condition. It can only be specified when backoffLimitPerIndex is set. It can be null or up to completions. It is required and must be less than or equal to 10^4 when is completions greater than 10^5. This field is beta-level. It can be used when the `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).
    #[serde(rename = "maxFailedIndexes", skip_serializing_if = "Option::is_none")]
    pub max_failed_indexes: Option<i32>,
    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(rename = "parallelism", skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(rename = "podFailurePolicy", skip_serializing_if = "Option::is_none")]
    pub pod_failure_policy: Option<Box<crate::models::V1PodFailurePolicy>>,
    /// podReplacementPolicy specifies when to create replacement Pods. Possible values are: - TerminatingOrFailed means that we recreate pods   when they are terminating (has a metadata.deletionTimestamp) or failed. - Failed means to wait until a previously created Pod is fully terminated (has phase   Failed or Succeeded) before creating a replacement Pod.  When using podFailurePolicy, Failed is the the only allowed value. TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use. This is an beta field. To use this, enable the JobPodReplacementPolicy feature toggle. This is on by default.
    #[serde(rename = "podReplacementPolicy", skip_serializing_if = "Option::is_none")]
    pub pod_replacement_policy: Option<String>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::models::V1LabelSelector>>,
    #[serde(rename = "successPolicy", skip_serializing_if = "Option::is_none")]
    pub success_policy: Option<Box<crate::models::V1SuccessPolicy>>,
    /// suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to true, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from false to true), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the StartTime field of the Job, effectively resetting the ActiveDeadlineSeconds timer too. Defaults to false.
    #[serde(rename = "suspend", skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    #[serde(rename = "template")]
    pub template: Box<crate::models::V1PodTemplateSpec>,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes.
    #[serde(rename = "ttlSecondsAfterFinished", skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<i32>,
}

impl V1JobSpec {
    /// JobSpec describes how the job execution will look like.
    pub fn new(template: crate::models::V1PodTemplateSpec) -> V1JobSpec {
        V1JobSpec {
            active_deadline_seconds: None,
            backoff_limit: None,
            backoff_limit_per_index: None,
            completion_mode: None,
            completions: None,
            managed_by: None,
            manual_selector: None,
            max_failed_indexes: None,
            parallelism: None,
            pod_failure_policy: None,
            pod_replacement_policy: None,
            selector: None,
            success_policy: None,
            suspend: None,
            template: Box::new(template),
            ttl_seconds_after_finished: None,
        }
    }
}


