/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1UserSubject : UserSubject holds detailed information for user-kind subject.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1UserSubject {
    /// `name` is the username that matches, or \"*\" to match all usernames. Required.
    #[serde(rename = "name")]
    pub name: String,
}

impl V1UserSubject {
    /// UserSubject holds detailed information for user-kind subject.
    pub fn new(name: String) -> V1UserSubject {
        V1UserSubject {
            name,
        }
    }
}


