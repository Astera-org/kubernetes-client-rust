/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1ServerStorageVersion : An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha1ServerStorageVersion {
    /// The ID of the reporting API server.
    #[serde(rename = "apiServerID", skip_serializing_if = "Option::is_none")]
    pub api_server_id: Option<String>,
    /// The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.
    #[serde(rename = "decodableVersions", skip_serializing_if = "Option::is_none")]
    pub decodable_versions: Option<Vec<String>>,
    /// The API server encodes the object to this version when persisting it in the backend (e.g., etcd).
    #[serde(rename = "encodingVersion", skip_serializing_if = "Option::is_none")]
    pub encoding_version: Option<String>,
    /// The API server can serve these versions. DecodableVersions must include all ServedVersions.
    #[serde(rename = "servedVersions", skip_serializing_if = "Option::is_none")]
    pub served_versions: Option<Vec<String>>,
}

impl V1alpha1ServerStorageVersion {
    /// An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.
    pub fn new() -> V1alpha1ServerStorageVersion {
        V1alpha1ServerStorageVersion {
            api_server_id: None,
            decodable_versions: None,
            encoding_version: None,
            served_versions: None,
        }
    }
}


