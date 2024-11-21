/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta3PriorityLevelConfigurationReference : PriorityLevelConfigurationReference contains information that points to the \"request-priority\" being used.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta3PriorityLevelConfigurationReference {
    /// `name` is the name of the priority level configuration being referenced Required.
    #[serde(rename = "name")]
    pub name: String,
}

impl V1beta3PriorityLevelConfigurationReference {
    /// PriorityLevelConfigurationReference contains information that points to the \"request-priority\" being used.
    pub fn new(name: String) -> V1beta3PriorityLevelConfigurationReference {
        V1beta3PriorityLevelConfigurationReference {
            name,
        }
    }
}


