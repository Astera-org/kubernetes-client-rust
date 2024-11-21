/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IngressTls : IngressTLS describes the transport layer security associated with an ingress.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1IngressTls {
    /// hosts is a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// secretName is the name of the secret used to terminate TLS traffic on port 443. Field is left optional to allow TLS routing based on SNI hostname alone. If the SNI host in a listener conflicts with the \"Host\" header field used by an IngressRule, the SNI host is used for termination and value of the \"Host\" header is used for routing.
    #[serde(rename = "secretName", skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}

impl V1IngressTls {
    /// IngressTLS describes the transport layer security associated with an ingress.
    pub fn new() -> V1IngressTls {
        V1IngressTls {
            hosts: None,
            secret_name: None,
        }
    }
}

