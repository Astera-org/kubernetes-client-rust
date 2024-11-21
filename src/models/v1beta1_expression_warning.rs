/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1ExpressionWarning : ExpressionWarning is a warning information that targets a specific expression.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1beta1ExpressionWarning {
    /// The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is \"spec.validations[0].expression\"
    #[serde(rename = "fieldRef")]
    pub field_ref: String,
    /// The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.
    #[serde(rename = "warning")]
    pub warning: String,
}

impl V1beta1ExpressionWarning {
    /// ExpressionWarning is a warning information that targets a specific expression.
    pub fn new(field_ref: String, warning: String) -> V1beta1ExpressionWarning {
        V1beta1ExpressionWarning {
            field_ref,
            warning,
        }
    }
}


