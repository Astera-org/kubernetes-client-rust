/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CronJobStatus : CronJobStatus represents the current state of a cron job.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CronJobStatus {
    /// A list of pointers to currently running jobs.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<crate::models::V1ObjectReference>>,
    /// Information when was the last time the job was successfully scheduled.
    #[serde(rename = "lastScheduleTime", skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<String>,
    /// Information when was the last time the job successfully completed.
    #[serde(rename = "lastSuccessfulTime", skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<String>,
}

impl V1CronJobStatus {
    /// CronJobStatus represents the current state of a cron job.
    pub fn new() -> V1CronJobStatus {
        V1CronJobStatus {
            active: None,
            last_schedule_time: None,
            last_successful_time: None,
        }
    }
}


