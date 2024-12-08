use k8s_openapi::api::core;
use kube::core::object::{HasSpec, HasStatus};
use std::cmp::Ordering;
use std::sync::Arc;

use crate::crd::{self, ConcurrencyPolicy};
use crate::error::Error;
use crate::util::{
    construct_job_for_cron_job, get_next_schedule, get_scheduled_time_for_job, is_job_finished,
};
use futures::StreamExt;
use k8s_openapi::api::batch;
use k8s_openapi::apimachinery::pkg::apis::meta;
use kube::api::{DeleteParams, ListParams, PostParams};
use kube::runtime::{watcher, Controller};
use kube::{runtime::controller::Action, Api};
use kube::{Client, Resource, ResourceExt};
use log::{error, info};
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct Context {
    /// Kubernetes client
    pub client: Client,
    pub clock: Arc<dyn Clock>,
}

/// The Clock trait defines how to get the current time.
pub trait Clock: Sync + Send {
    fn now(&self) -> SystemTime;
}

/// realClock implements the Clock trait and returns the actual system time.
#[derive(Clone)]
pub struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub async fn run(ctx: Arc<Context>) {
    let client = ctx.client.clone();
    let api: Api<crd::CronJob> = Api::all(client.clone());
    let job_api: Api<batch::v1::Job> = Api::all(client.clone());

    Controller::new(api.clone(), watcher::Config::default())
        .shutdown_on_signal()
        .owns(job_api, watcher::Config::default()) // Makes it so that OwnerReference is set and the controller watches the secondary resource
        .run(reconcile, error_policy, ctx)
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;
}

pub async fn reconcile(g: Arc<crd::CronJob>, ctx: Arc<Context>) -> Result<Action, Error> {
    info!("{:?}", ctx.as_ref().clock.now());
    let client = &ctx.client;

    let ns = g
        .as_ref()
        .metadata
        .namespace
        .clone()
        .unwrap_or("default".to_string());

    //////////////////////// 1: Load the CronJob by name ////////////////////////
    // This is required since the Arc<crd::CronJob> doesn't allow changes
    let cronjob_api: Api<crd::CronJob> = Api::namespaced(client.clone(), &ns);
    let mut cronjob = cronjob_api.get(&g.name_any()).await.map_err(|err| {
        error!("unable to list child Jobs: {err}");
        Error::KubeError(err)
    })?;

    //////////////////////// 2: List all active jobs, and update the status ////////////////////////

    let job_api: Api<batch::v1::Job> = Api::namespaced(client.clone(), &ns);
    // let field_selector: String = format!("jobOwnerKey={}", g.name_any());

    // find the active list of jobs
    let mut active_jobs = vec![];
    let mut successful_jobs = vec![];
    let mut failed_jobs = vec![];
    let mut most_recent_time: Option<SystemTime> = None;

    let job_list = job_api
        .list(
            &ListParams::default(), //.fields(&field_selector)
        )
        .await
        .map_err(|err| {
            error!("unable to list child Jobs: {err}");
            Error::KubeError(err)
        })?;

    for job in job_list {
        let scheduled_time_for_job =
            get_scheduled_time_for_job(&job).map_err(Error::ChronoParseError)?;

        if let Some(scheduled_time_for_job) = scheduled_time_for_job {
            if most_recent_time.map_or(true, |time| time < scheduled_time_for_job) {
                most_recent_time = Some(scheduled_time_for_job);
            }
        }

        let (_, status) = is_job_finished(&job);
        match status.as_str() {
            "" => active_jobs.push(job),
            "Complete" => successful_jobs.push(job),
            "Failed" => failed_jobs.push(job),
            _ => unreachable!(),
        }
    }

    if let Some(status) = cronjob.status_mut() {
        status.last_schedule_time = most_recent_time.map(|time| meta::v1::Time(time.into()));

        let mut active: Vec<core::v1::ObjectReference> = vec![];
        for active_job in &active_jobs {
            active.push(active_job.object_ref(&()));
        }

        status.active = Some(active);
    }

    info!(
        "\"job count\", \"active jobs\", {}, \"successful jobs\", {}, \"failed jobs\", {}",
        active_jobs.len(),
        successful_jobs.len(),
        failed_jobs.len()
    );

    //////////////////////// 3: Clean up old jobs according to the history limit ////////////////////////
    // NB: deleting these are "best effort" -- if we fail on a particular one,
    // we won't requeue just to finish the deleting.
    if let Some(failed_jobs_history_limit) = cronjob.spec().failed_jobs_history_limit {
        failed_jobs.sort_by(|a, b| {
            match (
                a.status.as_ref().map(|status| status.start_time.as_ref()),
                b.status.as_ref().map(|status| status.start_time.as_ref()),
            ) {
                (Some(_), None) => Ordering::Less, // a comes before b if b has no start time
                (None, Some(_)) => Ordering::Greater, // b comes before a if a has no start time
                (Some(start_a), Some(start_b)) => start_a.cmp(&start_b), // Compare the start times
                (None, None) => Ordering::Equal,   // Both are None (equal)
            }
        });

        for (i, job) in failed_jobs.iter().enumerate() {
            if i as i32 >= failed_jobs.len() as i32 - failed_jobs_history_limit {
                break;
            }
            match job_api
                .delete(&job.name_any(), &DeleteParams::background())
                .await
            {
                Ok(_) => info!("deleted old failed job: {:?}", job),
                Err(err) => error!("unable to delete old failed job: {:?}", err),
            };
        }
    }

    if let Some(successful_jobs_history_limit) = cronjob.spec().successful_jobs_history_limit {
        successful_jobs.sort_by(|a, b| {
            match (
                a.status.as_ref().map(|status| status.start_time.as_ref()),
                b.status.as_ref().map(|status| status.start_time.as_ref()),
            ) {
                (Some(_), None) => Ordering::Less, // a comes before b if b has no start time
                (None, Some(_)) => Ordering::Greater, // b comes before a if a has no start time
                (Some(start_a), Some(start_b)) => start_a.cmp(&start_b), // Compare the start times
                (None, None) => Ordering::Equal,   // Both are None (equal)
            }
        });

        for (i, job) in successful_jobs.iter().enumerate() {
            if i as i32 >= successful_jobs.len() as i32 - successful_jobs_history_limit {
                break;
            }
            match job_api
                .delete(&job.name_any(), &DeleteParams::background())
                .await
            {
                Ok(_) => info!("deleted old successful job: {:?}", job),
                Err(err) => error!("unable to delete old successful job: {:?}", err),
            };
        }
    }

    //////////////////////// 4: Check if we're suspended ////////////////////////
    if cronjob.spec().suspend.unwrap_or(false) {
        info!("cronjob suspended, skipping");
        return Ok(Action::await_change());
    }

    //////////////////////// 5: Get the next scheduled run ////////////////////////
    // figure out the next times that we need to create
    // jobs at (or anything we missed).
    let (missed_run, next_run) = match get_next_schedule(&cronjob, ctx.clock.now()) {
        Ok((missed_run, next_run)) => (missed_run, next_run),
        // we don't really care about requeuing until we get an update that
        // fixes the schedule, so don't return an error
        Err(err) => {
            error!("unable to figure out CronJob schedule: {err}");
            return Ok(Action::await_change());
        }
    };

    let scheduled_result: Result<Action, Error> = Ok(Action::requeue(
        next_run
            .duration_since(ctx.clock.now())
            .map_err(Error::SystemTimeError)?,
    ));
    info!("Now: {:?}\nNext run: {:?}", ctx.clock.now(), next_run); // TODO solve the log.WithValues difference

    //////////////////////// 6: Run a new job if it's on schedule, not past the deadline, and not blocked by our concurrency policy ////////////////////////
    if missed_run.is_none() {
        // Not sure if this is a correct translation
        info!("no upcoming scheduled times, sleeping until next");
        return scheduled_result;
    }

    let missed_run = missed_run.unwrap();

    // make sure we're not too late to start the run
    info!("current run: {:?}", missed_run);
    if let Some(starting_deadline_seconds) = cronjob.spec().starting_deadline_seconds {
        if (missed_run + Duration::from_secs(starting_deadline_seconds as u64)) < ctx.clock.now() {
            info!("missed starting deadline for last run, sleeping till next");
            // TODO(directxman12): events
            return scheduled_result;
        }
    }

    // figure out how to run this job -- concurrency policy might forbid us from running
    // multiple at the same time...
    if cronjob.spec().concurrency_policy == ConcurrencyPolicy::Forbid && !active_jobs.is_empty() {
        info!(
            "concurrency policy blocks concurrent runs, skipping\nnum active: {}",
            active_jobs.len()
        );
        return scheduled_result;
    }

    // ...or instruct us to replace existing ones...
    if cronjob.spec().concurrency_policy == ConcurrencyPolicy::Replace {
        for job in active_jobs {
            // we don't care if the job was already deleted
            if let Err(err) = job_api
                .delete(&job.name_any(), &DeleteParams::background())
                .await
            {
                error!("Unable to delete active job: {:?}", job);
                return Err(Error::KubeError(err));
            }
        }
    }

    // actually make the job...
    let job = match construct_job_for_cron_job(&cronjob, missed_run) {
        Ok(job) => job,
        Err(_) => {
            error!("unable to create job from template");
            // don't bother requeuing until we get a change to the spec
            return scheduled_result;
        }
    };

    // ...and create it on the cluster
    let job_cluster = job_api
        .create(&PostParams::default(), &job)
        .await
        .map_err(Error::KubeError)?;

    info!("Created job for CronJob run: {:?}", job_cluster);

    scheduled_result
}

pub fn error_policy(_obj: Arc<crd::CronJob>, _error: &Error, _ctx: Arc<Context>) -> Action {
    Action::requeue(Duration::from_secs(60))
}
