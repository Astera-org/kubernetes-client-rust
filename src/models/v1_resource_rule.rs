/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ResourceRule : ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ResourceRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  \"*\" means all.
    #[serde(rename = "apiGroups", skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<Vec<String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  \"*\" means all.
    #[serde(rename = "resourceNames", skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<Vec<String>>,
    /// Resources is a list of resources this rule applies to.  \"*\" means all in the specified apiGroups.  \"*_/foo\" represents the subresource 'foo' for all resources in the specified apiGroups.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  \"*\" means all.
    #[serde(rename = "verbs")]
    pub verbs: Vec<String>,
}

impl V1ResourceRule {
    /// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    pub fn new(verbs: Vec<String>) -> V1ResourceRule {
        V1ResourceRule {
            api_groups: None,
            resource_names: None,
            resources: None,
            verbs,
        }
    }
}


