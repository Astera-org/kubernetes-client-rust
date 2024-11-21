/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PodSpec : PodSpec is a description of a pod.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1PodSpec {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    #[serde(rename = "activeDeadlineSeconds", skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Box<crate::models::V1Affinity>>,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(rename = "automountServiceAccountToken", skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
    /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
    #[serde(rename = "containers")]
    pub containers: Vec<crate::models::V1Container>,
    #[serde(rename = "dnsConfig", skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<Box<crate::models::V1PodDnsConfig>>,
    /// Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    #[serde(rename = "dnsPolicy", skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
    /// EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
    #[serde(rename = "enableServiceLinks", skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,
    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
    #[serde(rename = "ephemeralContainers", skip_serializing_if = "Option::is_none")]
    pub ephemeral_containers: Option<Vec<crate::models::V1EphemeralContainer>>,
    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified.
    #[serde(rename = "hostAliases", skip_serializing_if = "Option::is_none")]
    pub host_aliases: Option<Vec<crate::models::V1HostAlias>>,
    /// Use the host's ipc namespace. Optional: Default to false.
    #[serde(rename = "hostIPC", skip_serializing_if = "Option::is_none")]
    pub host_ipc: Option<bool>,
    /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
    #[serde(rename = "hostNetwork", skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// Use the host's pid namespace. Optional: Default to false.
    #[serde(rename = "hostPID", skip_serializing_if = "Option::is_none")]
    pub host_pid: Option<bool>,
    /// Use the host's user namespace. Optional: Default to true. If set to true or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to false, a new userns is created for the pod. Setting false is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host. This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.
    #[serde(rename = "hostUsers", skip_serializing_if = "Option::is_none")]
    pub host_users: Option<bool>,
    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    #[serde(rename = "imagePullSecrets", skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<Vec<crate::models::V1LocalObjectReference>>,
    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    #[serde(rename = "initContainers", skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<crate::models::V1Container>>,
    /// NodeName indicates in which node this pod is scheduled. If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName. Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod. This field should not be used to express a desire for the pod to be scheduled on a specific node. https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename
    #[serde(rename = "nodeName", skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    #[serde(rename = "nodeSelector", skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<Box<crate::models::V1PodOs>>,
    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md
    #[serde(rename = "overhead", skip_serializing_if = "Option::is_none")]
    pub overhead: Option<::std::collections::HashMap<String, String>>,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset.
    #[serde(rename = "preemptionPolicy", skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<String>,
    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// If specified, indicates the pod's priority. \"system-node-critical\" and \"system-cluster-critical\" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    #[serde(rename = "priorityClassName", skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<String>,
    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to \"True\" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates
    #[serde(rename = "readinessGates", skip_serializing_if = "Option::is_none")]
    pub readiness_gates: Option<Vec<crate::models::V1PodReadinessGate>>,
    /// ResourceClaims defines which ResourceClaims must be allocated and reserved before the Pod is allowed to start. The resources will be made available to those containers which consume them by name.  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.  This field is immutable.
    #[serde(rename = "resourceClaims", skip_serializing_if = "Option::is_none")]
    pub resource_claims: Option<Vec<crate::models::V1PodResourceClaim>>,
    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
    #[serde(rename = "restartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the \"legacy\" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
    #[serde(rename = "runtimeClassName", skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,
    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    #[serde(rename = "schedulerName", skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<String>,
    /// SchedulingGates is an opaque list of values that if specified will block scheduling the pod. If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the scheduler will not attempt to schedule the pod.  SchedulingGates can only be set at pod creation time, and be removed only afterwards.
    #[serde(rename = "schedulingGates", skip_serializing_if = "Option::is_none")]
    pub scheduling_gates: Option<Vec<crate::models::V1PodSchedulingGate>>,
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<Box<crate::models::V1PodSecurityContext>>,
    /// DeprecatedServiceAccount is a deprecated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    /// If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.
    #[serde(rename = "setHostnameAsFQDN", skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: Option<bool>,
    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.
    #[serde(rename = "shareProcessNamespace", skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    /// If specified, the fully qualified Pod hostname will be \"<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>\". If not specified, the pod will not have a domainname at all.
    #[serde(rename = "subdomain", skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    #[serde(rename = "terminationGracePeriodSeconds", skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// If specified, the pod's tolerations.
    #[serde(rename = "tolerations", skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<crate::models::V1Toleration>>,
    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.
    #[serde(rename = "topologySpreadConstraints", skip_serializing_if = "Option::is_none")]
    pub topology_spread_constraints: Option<Vec<crate::models::V1TopologySpreadConstraint>>,
    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<crate::models::V1Volume>>,
}

impl V1PodSpec {
    /// PodSpec is a description of a pod.
    pub fn new(containers: Vec<crate::models::V1Container>) -> V1PodSpec {
        V1PodSpec {
            active_deadline_seconds: None,
            affinity: None,
            automount_service_account_token: None,
            containers,
            dns_config: None,
            dns_policy: None,
            enable_service_links: None,
            ephemeral_containers: None,
            host_aliases: None,
            host_ipc: None,
            host_network: None,
            host_pid: None,
            host_users: None,
            hostname: None,
            image_pull_secrets: None,
            init_containers: None,
            node_name: None,
            node_selector: None,
            os: None,
            overhead: None,
            preemption_policy: None,
            priority: None,
            priority_class_name: None,
            readiness_gates: None,
            resource_claims: None,
            restart_policy: None,
            runtime_class_name: None,
            scheduler_name: None,
            scheduling_gates: None,
            security_context: None,
            service_account: None,
            service_account_name: None,
            set_hostname_as_fqdn: None,
            share_process_namespace: None,
            subdomain: None,
            termination_grace_period_seconds: None,
            tolerations: None,
            topology_spread_constraints: None,
            volumes: None,
        }
    }
}

