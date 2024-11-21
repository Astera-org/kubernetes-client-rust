/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1MatchResources : MatchResources decides whether to run the admission control policy on an object based on whether it meets the match criteria. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha1MatchResources {
    /// ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
    #[serde(rename = "excludeResourceRules", skip_serializing_if = "Option::is_none")]
    pub exclude_resource_rules: Option<Vec<crate::models::V1alpha1NamedRuleWithOperations>>,
    /// matchPolicy defines how the \"MatchResources\" list is used to match incoming requests. Allowed values are \"Exact\" or \"Equivalent\".  - Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the ValidatingAdmissionPolicy.  - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the ValidatingAdmissionPolicy.  Defaults to \"Equivalent\"
    #[serde(rename = "matchPolicy", skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<String>,
    #[serde(rename = "namespaceSelector", skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<Box<crate::models::V1LabelSelector>>,
    #[serde(rename = "objectSelector", skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<Box<crate::models::V1LabelSelector>>,
    /// ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches. The policy cares about an operation if it matches _any_ Rule.
    #[serde(rename = "resourceRules", skip_serializing_if = "Option::is_none")]
    pub resource_rules: Option<Vec<crate::models::V1alpha1NamedRuleWithOperations>>,
}

impl V1alpha1MatchResources {
    /// MatchResources decides whether to run the admission control policy on an object based on whether it meets the match criteria. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
    pub fn new() -> V1alpha1MatchResources {
        V1alpha1MatchResources {
            exclude_resource_rules: None,
            match_policy: None,
            namespace_selector: None,
            object_selector: None,
            resource_rules: None,
        }
    }
}


