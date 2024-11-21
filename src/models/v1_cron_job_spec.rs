/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.31.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CronJobSpec : CronJobSpec describes how the job execution will look like and when it will actually run.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are:  - \"Allow\" (default): allows CronJobs to run concurrently; - \"Forbid\": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - \"Replace\": cancels currently running job and replaces it with a new one
    #[serde(rename = "concurrencyPolicy", skip_serializing_if = "Option::is_none")]
    pub concurrency_policy: Option<String>,
    /// The number of failed finished jobs to retain. Value must be non-negative integer. Defaults to 1.
    #[serde(rename = "failedJobsHistoryLimit", skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
    #[serde(rename = "jobTemplate")]
    pub job_template: Box<crate::models::V1JobTemplateSpec>,
    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    #[serde(rename = "startingDeadlineSeconds", skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,
    /// The number of successful finished jobs to retain. Value must be non-negative integer. Defaults to 3.
    #[serde(rename = "successfulJobsHistoryLimit", skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
    #[serde(rename = "suspend", skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// The time zone name for the given schedule, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. If not specified, this will default to the time zone of the kube-controller-manager process. The set of valid time zone names and the time zone offset is loaded from the system-wide time zone database by the API server during CronJob validation and the controller manager during execution. If no system-wide time zone database can be found a bundled version of the database is used instead. If the time zone name becomes invalid during the lifetime of a CronJob or due to a change in host configuration, the controller will stop creating new new Jobs and will create a system event with the reason UnknownTimeZone. More information can be found in https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#time-zones
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl V1CronJobSpec {
    /// CronJobSpec describes how the job execution will look like and when it will actually run.
    pub fn new(job_template: crate::models::V1JobTemplateSpec, schedule: String) -> V1CronJobSpec {
        V1CronJobSpec {
            concurrency_policy: None,
            failed_jobs_history_limit: None,
            job_template: Box::new(job_template),
            schedule,
            starting_deadline_seconds: None,
            successful_jobs_history_limit: None,
            suspend: None,
            time_zone: None,
        }
    }
}


