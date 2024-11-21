/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IpBlock : IPBlock describes a particular CIDR (Ex. \"192.168.1.0/24\",\"2001:db8::/64\") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1IpBlock {
    /// cidr is a string representing the IPBlock Valid examples are \"192.168.1.0/24\" or \"2001:db8::/64\"
    #[serde(rename = "cidr")]
    pub cidr: String,
    /// except is a slice of CIDRs that should not be included within an IPBlock Valid examples are \"192.168.1.0/24\" or \"2001:db8::/64\" Except values will be rejected if they are outside the cidr range
    #[serde(rename = "except", skip_serializing_if = "Option::is_none")]
    pub except: Option<Vec<String>>,
}

impl V1IpBlock {
    /// IPBlock describes a particular CIDR (Ex. \"192.168.1.0/24\",\"2001:db8::/64\") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
    pub fn new(cidr: String) -> V1IpBlock {
        V1IpBlock {
            cidr,
            except: None,
        }
    }
}

