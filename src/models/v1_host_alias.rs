/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1HostAlias : HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1HostAlias {
    /// Hostnames for the above IP address.
    #[serde(rename = "hostnames", skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// IP address of the host file entry.
    #[serde(rename = "ip")]
    pub ip: String,
}

impl V1HostAlias {
    /// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
    pub fn new(ip: String) -> V1HostAlias {
        V1HostAlias {
            hostnames: None,
            ip,
        }
    }
}


