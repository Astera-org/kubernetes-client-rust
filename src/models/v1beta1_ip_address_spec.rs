/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1IpAddressSpec : IPAddressSpec describe the attributes in an IP Address.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1IpAddressSpec {
    #[serde(rename = "parentRef")]
    pub parent_ref: Box<crate::models::V1beta1ParentReference>,
}

impl V1beta1IpAddressSpec {
    /// IPAddressSpec describe the attributes in an IP Address.
    pub fn new(parent_ref: crate::models::V1beta1ParentReference) -> V1beta1IpAddressSpec {
        V1beta1IpAddressSpec {
            parent_ref: Box::new(parent_ref),
        }
    }
}


