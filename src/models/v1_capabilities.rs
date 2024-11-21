/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Capabilities : Adds and removes POSIX capabilities from running containers.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1Capabilities {
    /// Added capabilities
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// Removed capabilities
    #[serde(rename = "drop", skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

impl V1Capabilities {
    /// Adds and removes POSIX capabilities from running containers.
    pub fn new() -> V1Capabilities {
        V1Capabilities {
            add: None,
            drop: None,
        }
    }
}


