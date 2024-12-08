use k8s_openapi::{
    api::{batch, core},
    apimachinery::pkg::apis::meta,
};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// #[serde(skip_serializing_if = "Option::is_none")]

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(
    group = "batch.tutorial.kube.rs",
    version = "v1",
    kind = "CronJob",
    namespaced
)]
#[kube(status = "CronJobStatus", shortname = "test-name")]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpec {
    pub schedule: String,
    pub starting_deadline_seconds: Option<i64>,
    pub concurrency_policy: ConcurrencyPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    pub job_template: batch::v1::JobTemplateSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<core::v1::ObjectReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<meta::v1::Time>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone, JsonSchema)]
pub enum ConcurrencyPolicy {
    #[default]
    Allow,
    Forbid,
    Replace,
}
