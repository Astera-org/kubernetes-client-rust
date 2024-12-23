/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CustomResourceValidation : CustomResourceValidation is a list of validation methods for CustomResources.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CustomResourceValidation {
    #[serde(rename = "openAPIV3Schema", skip_serializing_if = "Option::is_none")]
    pub open_apiv3_schema: Option<Box<crate::models::V1JsonSchemaProps>>,
}

impl V1CustomResourceValidation {
    /// CustomResourceValidation is a list of validation methods for CustomResources.
    pub fn new() -> V1CustomResourceValidation {
        V1CustomResourceValidation {
            open_apiv3_schema: None,
        }
    }
}


