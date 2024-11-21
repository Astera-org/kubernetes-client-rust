/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IngressPortStatus : IngressPortStatus represents the error condition of a service port



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1IngressPortStatus {
    /// error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use   CamelCase names - cloud provider specific error values must have names that comply with the   format foo.example.com/CamelCase.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// port is the port number of the ingress port.
    #[serde(rename = "port")]
    pub port: i32,
    /// protocol is the protocol of the ingress port. The supported values are: \"TCP\", \"UDP\", \"SCTP\"
    #[serde(rename = "protocol")]
    pub protocol: String,
}

impl V1IngressPortStatus {
    /// IngressPortStatus represents the error condition of a service port
    pub fn new(port: i32, protocol: String) -> V1IngressPortStatus {
        V1IngressPortStatus {
            error: None,
            port,
            protocol,
        }
    }
}


