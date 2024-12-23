/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NonResourceRule : NonResourceRule holds information that describes a rule for the non-resource



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NonResourceRule {
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  \"*\" means all.
    #[serde(rename = "nonResourceURLs", skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<Vec<String>>,
    /// Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  \"*\" means all.
    #[serde(rename = "verbs")]
    pub verbs: Vec<String>,
}

impl V1NonResourceRule {
    /// NonResourceRule holds information that describes a rule for the non-resource
    pub fn new(verbs: Vec<String>) -> V1NonResourceRule {
        V1NonResourceRule {
            non_resource_urls: None,
            verbs,
        }
    }
}


