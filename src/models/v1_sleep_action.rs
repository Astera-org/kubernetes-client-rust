/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1SleepAction : SleepAction describes a \"sleep\" action.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1SleepAction {
    /// Seconds is the number of seconds to sleep.
    #[serde(rename = "seconds")]
    pub seconds: i64,
}

impl V1SleepAction {
    /// SleepAction describes a \"sleep\" action.
    pub fn new(seconds: i64) -> V1SleepAction {
        V1SleepAction {
            seconds,
        }
    }
}

