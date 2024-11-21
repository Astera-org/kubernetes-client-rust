/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Endpoint : Endpoint represents a single logical \"backend\" implementing a service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1Endpoint {
    /// addresses of this endpoint. The contents of this field are interpreted according to the corresponding EndpointSlice addressType field. Consumers must handle different types of addresses in the context of their own capabilities. This must contain at least one address but no more than 100. These are all assumed to be fungible and clients may choose to only use the first element. Refer to: https://issue.k8s.io/106267
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Box<crate::models::V1EndpointConditions>>,
    /// deprecatedTopology contains topology information part of the v1beta1 API. This field is deprecated, and will be removed when the v1beta1 API is removed (no sooner than kubernetes v1.24).  While this field can hold values, it is not writable through the v1 API, and any attempts to write to it will be silently ignored. Topology information can be found in the zone and nodeName fields instead.
    #[serde(rename = "deprecatedTopology", skip_serializing_if = "Option::is_none")]
    pub deprecated_topology: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "hints", skip_serializing_if = "Option::is_none")]
    pub hints: Option<Box<crate::models::V1EndpointHints>>,
    /// hostname of this endpoint. This field may be used by consumers of endpoints to distinguish endpoints from each other (e.g. in DNS names). Multiple endpoints which use the same hostname should be considered fungible (e.g. multiple A values in DNS). Must be lowercase and pass DNS Label (RFC 1123) validation.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// nodeName represents the name of the Node hosting this endpoint. This can be used to determine endpoints local to a Node.
    #[serde(rename = "nodeName", skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "targetRef", skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<Box<crate::models::V1ObjectReference>>,
    /// zone is the name of the Zone this endpoint exists in.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl V1Endpoint {
    /// Endpoint represents a single logical \"backend\" implementing a service.
    pub fn new(addresses: Vec<String>) -> V1Endpoint {
        V1Endpoint {
            addresses,
            conditions: None,
            deprecated_topology: None,
            hints: None,
            hostname: None,
            node_name: None,
            target_ref: None,
            zone: None,
        }
    }
}

