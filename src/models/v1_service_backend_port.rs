/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ServiceBackendPort : ServiceBackendPort is the service port being referenced.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1ServiceBackendPort {
    /// name is the name of the port on the Service. This is a mutually exclusive setting with \"Number\".
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with \"Name\".
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

impl V1ServiceBackendPort {
    /// ServiceBackendPort is the service port being referenced.
    pub fn new() -> V1ServiceBackendPort {
        V1ServiceBackendPort {
            name: None,
            number: None,
        }
    }
}

