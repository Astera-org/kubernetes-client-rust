/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1RoleRef : RoleRef contains information that points to the role being used



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1RoleRef {
    /// APIGroup is the group for the resource being referenced
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    /// Kind is the type of resource being referenced
    #[serde(rename = "kind")]
    pub kind: String,
    /// Name is the name of resource being referenced
    #[serde(rename = "name")]
    pub name: String,
}

impl V1RoleRef {
    /// RoleRef contains information that points to the role being used
    pub fn new(api_group: String, kind: String, name: String) -> V1RoleRef {
        V1RoleRef {
            api_group,
            kind,
            name,
        }
    }
}


