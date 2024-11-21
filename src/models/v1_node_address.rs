/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NodeAddress : NodeAddress contains information for the node's address.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1NodeAddress {
    /// The node address.
    #[serde(rename = "address")]
    pub address: String,
    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1NodeAddress {
    /// NodeAddress contains information for the node's address.
    pub fn new(address: String, _type: String) -> V1NodeAddress {
        V1NodeAddress {
            address,
            _type,
        }
    }
}

