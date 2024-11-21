/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1EphemeralVolumeSource : Represents an ephemeral volume that is handled by a normal storage driver.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1EphemeralVolumeSource {
    #[serde(rename = "volumeClaimTemplate", skip_serializing_if = "Option::is_none")]
    pub volume_claim_template: Option<Box<crate::models::V1PersistentVolumeClaimTemplate>>,
}

impl V1EphemeralVolumeSource {
    /// Represents an ephemeral volume that is handled by a normal storage driver.
    pub fn new() -> V1EphemeralVolumeSource {
        V1EphemeralVolumeSource {
            volume_claim_template: None,
        }
    }
}


