/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IngressSpec : IngressSpec describes the Ingress the user wishes to exist.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1IngressSpec {
    #[serde(rename = "defaultBackend", skip_serializing_if = "Option::is_none")]
    pub default_backend: Option<Box<crate::models::V1IngressBackend>>,
    /// ingressClassName is the name of an IngressClass cluster resource. Ingress controller implementations use this field to know whether they should be serving this Ingress resource, by a transitive connection (controller -> IngressClass -> Ingress resource). Although the `kubernetes.io/ingress.class` annotation (simple constant name) was never formally defined, it was widely supported by Ingress controllers to create a direct binding between Ingress controller and Ingress resources. Newly created Ingress resources should prefer using the field. However, even though the annotation is officially deprecated, for backwards compatibility reasons, ingress controllers should still honor that annotation if present.
    #[serde(rename = "ingressClassName", skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    /// rules is a list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::V1IngressRule>>,
    /// tls represents the TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<crate::models::V1IngressTls>>,
}

impl V1IngressSpec {
    /// IngressSpec describes the Ingress the user wishes to exist.
    pub fn new() -> V1IngressSpec {
        V1IngressSpec {
            default_backend: None,
            ingress_class_name: None,
            rules: None,
            tls: None,
        }
    }
}


