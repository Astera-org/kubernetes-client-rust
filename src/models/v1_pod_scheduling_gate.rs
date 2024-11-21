/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PodSchedulingGate : PodSchedulingGate is associated to a Pod to guard its scheduling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1PodSchedulingGate {
    /// Name of the scheduling gate. Each scheduling gate must have a unique name field.
    #[serde(rename = "name")]
    pub name: String,
}

impl V1PodSchedulingGate {
    /// PodSchedulingGate is associated to a Pod to guard its scheduling.
    pub fn new(name: String) -> V1PodSchedulingGate {
        V1PodSchedulingGate {
            name,
        }
    }
}

