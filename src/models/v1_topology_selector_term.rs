/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1TopologySelectorTerm : A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1TopologySelectorTerm {
    /// A list of topology selector requirements by labels.
    #[serde(rename = "matchLabelExpressions", skip_serializing_if = "Option::is_none")]
    pub match_label_expressions: Option<Vec<crate::models::V1TopologySelectorLabelRequirement>>,
}

impl V1TopologySelectorTerm {
    /// A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.
    pub fn new() -> V1TopologySelectorTerm {
        V1TopologySelectorTerm {
            match_label_expressions: None,
        }
    }
}


