/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha3DeviceAttribute : DeviceAttribute must have exactly one field set.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha3DeviceAttribute {
    /// BoolValue is a true/false value.
    #[serde(rename = "bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// IntValue is a number.
    #[serde(rename = "int", skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    /// StringValue is a string. Must not be longer than 64 characters.
    #[serde(rename = "string", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl V1alpha3DeviceAttribute {
    /// DeviceAttribute must have exactly one field set.
    pub fn new() -> V1alpha3DeviceAttribute {
        V1alpha3DeviceAttribute {
            bool: None,
            int: None,
            string: None,
            version: None,
        }
    }
}

