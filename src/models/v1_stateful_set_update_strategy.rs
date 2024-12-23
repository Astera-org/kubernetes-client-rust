/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1StatefulSetUpdateStrategy : StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1StatefulSetUpdateStrategy {
    #[serde(rename = "rollingUpdate", skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<Box<crate::models::V1RollingUpdateStatefulSetStrategy>>,
    /// Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl V1StatefulSetUpdateStrategy {
    /// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
    pub fn new() -> V1StatefulSetUpdateStrategy {
        V1StatefulSetUpdateStrategy {
            rolling_update: None,
            _type: None,
        }
    }
}


