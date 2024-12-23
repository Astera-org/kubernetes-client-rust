/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1DaemonEndpoint : DaemonEndpoint contains information about a single Daemon endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1DaemonEndpoint {
    /// Port number of the given endpoint.
    #[serde(rename = "Port")]
    pub port: i32,
}

impl V1DaemonEndpoint {
    /// DaemonEndpoint contains information about a single Daemon endpoint.
    pub fn new(port: i32) -> V1DaemonEndpoint {
        V1DaemonEndpoint {
            port,
        }
    }
}


