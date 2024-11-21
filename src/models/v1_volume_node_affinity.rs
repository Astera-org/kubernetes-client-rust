/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1VolumeNodeAffinity : VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1VolumeNodeAffinity {
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Box<crate::models::V1NodeSelector>>,
}

impl V1VolumeNodeAffinity {
    /// VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
    pub fn new() -> V1VolumeNodeAffinity {
        V1VolumeNodeAffinity {
            required: None,
        }
    }
}


