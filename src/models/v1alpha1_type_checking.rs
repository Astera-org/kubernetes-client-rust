/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1TypeChecking : TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1alpha1TypeChecking {
    /// The type checking warnings for each expression.
    #[serde(rename = "expressionWarnings", skip_serializing_if = "Option::is_none")]
    pub expression_warnings: Option<Vec<crate::models::V1alpha1ExpressionWarning>>,
}

impl V1alpha1TypeChecking {
    /// TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy
    pub fn new() -> V1alpha1TypeChecking {
        V1alpha1TypeChecking {
            expression_warnings: None,
        }
    }
}

