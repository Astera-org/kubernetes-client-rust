/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ForZone : ForZone provides information about which zones should consume this endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ForZone {
    /// name represents the name of the zone.
    #[serde(rename = "name")]
    pub name: String,
}

impl V1ForZone {
    /// ForZone provides information about which zones should consume this endpoint.
    pub fn new(name: String) -> V1ForZone {
        V1ForZone {
            name,
        }
    }
}


