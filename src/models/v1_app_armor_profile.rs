/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1AppArmorProfile : AppArmorProfile defines a pod or container's AppArmor settings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1AppArmorProfile {
    /// localhostProfile indicates a profile loaded on the node that should be used. The profile must be preconfigured on the node to work. Must match the loaded name of the profile. Must be set if and only if type is \"Localhost\".
    #[serde(rename = "localhostProfile", skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<String>,
    /// type indicates which kind of AppArmor profile will be applied. Valid options are:   Localhost - a profile pre-loaded on the node.   RuntimeDefault - the container runtime's default profile.   Unconfined - no AppArmor enforcement.
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1AppArmorProfile {
    /// AppArmorProfile defines a pod or container's AppArmor settings.
    pub fn new(_type: String) -> V1AppArmorProfile {
        V1AppArmorProfile {
            localhost_profile: None,
            _type,
        }
    }
}


