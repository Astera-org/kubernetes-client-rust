/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ContainerImage : Describe a container image



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ContainerImage {
    /// Names by which this image is known. e.g. [\"kubernetes.example/hyperkube:v1.0.7\", \"cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7\"]
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// The size of the image in bytes.
    #[serde(rename = "sizeBytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

impl V1ContainerImage {
    /// Describe a container image
    pub fn new() -> V1ContainerImage {
        V1ContainerImage {
            names: None,
            size_bytes: None,
        }
    }
}

