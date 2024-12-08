use cron::Schedule;
use k8s_openapi::{
    apimachinery::pkg::apis::meta,
    chrono::{Local, ParseError},
};
use std::{
    collections::BTreeMap,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use k8s_openapi::{api::batch::v1::Job, chrono::DateTime};
use kube::{api::ObjectMeta, Resource, ResourceExt};

use log::debug;

use crate::{
    crd::{self, CronJob},
    error::Error,
};

pub fn is_job_finished(job: &Job) -> (bool, String) {
    if let Some(status) = &job.status {
        if let Some(conditions) = status.conditions.as_ref() {
            if let Some(matching_condition) = conditions.iter().find(|cond| {
                (cond.type_ == "Complete" || cond.type_ == "Failed") && cond.status == "True"
            }) {
                return (true, matching_condition.type_.clone());
            }
        }
    }
    (
        false,
        "".to_string(), // It is called a JobCondition in the Go code, but in reality it doesn't follow it
                        // "" is not a valid entry in the enum, but since it is never used as a real JobCondition,
                        // it doesn't matter
    )
}

pub fn get_scheduled_time_for_job(job: &Job) -> Result<Option<SystemTime>, ParseError> {
    let time_raw = job.annotations().get("scheduledTimeAnnotation");
    let time = match time_raw {
        Some(time) if !time.is_empty() => time.to_string(), // If time is not empty, return it
        _ => {
            debug!("Job: {}, - Time was empty: {:?}", job.name_any(), time_raw);
            return Ok(None);
        }
    };
    let date_time = DateTime::parse_from_rfc3339(&time)?;
    Ok(Some(SystemTime::from(date_time)))
}

pub fn get_next_schedule(
    cronjob: &crd::CronJob,
    now: SystemTime,
) -> Result<(Option<SystemTime>, SystemTime), Error> {
    let schedule =
        Schedule::from_str(&cronjob.spec.schedule).map_err(Error::CronScheduleParseError)?;

    // for optimization purposes, cheat a bit and start from our last observed run time
    // we could reconstitute this here, but there's not much point, since we've
    // just updated it.
    let mut earliest_time: Option<meta::v1::Time> = None;
    if let Some(status) = &cronjob.status {
        earliest_time = Some(
            status
                .last_schedule_time
                .clone()
                .unwrap_or(
                    cronjob
                        .metadata
                        .creation_timestamp
                        .clone()
                        .expect("Job should be created and thus have a creation_timestamp"),
                )
                .clone(),
        );
    };

    if let Some(starting_deadline_seconds) = cronjob.spec.starting_deadline_seconds {
        if let Some(scheduling_deadline) = now
            .checked_sub(Duration::from_secs(
                starting_deadline_seconds
                    .try_into()
                    .expect("starting deadline seconds should be positive"),
            ))
            .map(|time| meta::v1::Time(time.into()))
        {
            // Compare scheduling_deadline with earliest_time
            if earliest_time.is_none()
                || scheduling_deadline
                    > earliest_time
                        .clone()
                        .expect("Checked by first part of it statement")
            {
                earliest_time = Some(scheduling_deadline);
            }
        }
    };

    // Not good practice, but easiest solution
    let earliest_time = if let Some(time) = &earliest_time {
        time.clone()
    } else {
        return Err(Error::MissingEarliestTime);
    };

    let schedule_next_now = schedule
        .after(&DateTime::<Local>::from(now))
        .next()
        .expect("Cron schedule should always have a next entry");
    let schedule_next_now_system = SystemTime::from(schedule_next_now);
    if earliest_time > meta::v1::Time(now.into()) {
        return Ok((None, schedule_next_now_system));
    };

    let mut starts = 0;
    let mut last_missed: Option<SystemTime> = None;

    for time in schedule
        .after(&earliest_time.0)
        .map(SystemTime::from)
        .take_while(|time| time <= &now)
    {
        last_missed = Some(time);
        starts += 1;
        if starts > 100 {
            return Err(Error::TooManyMissedStarts);
        }
    }

    Ok((last_missed, schedule_next_now_system))
}

pub fn construct_job_for_cron_job(
    cronjob: &CronJob,
    scheduled_time: SystemTime,
) -> Result<Job, Error> {
    // We want job names for a given nominal start time to have a deterministic name to avoid the same job being created twice
    let name = format!(
        "{}-{}",
        cronjob.name_any(),
        scheduled_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
    );

    let mut job = Job {
        metadata: ObjectMeta {
            labels: Some(BTreeMap::new()),
            annotations: Some(BTreeMap::new()),
            name: Some(name),
            namespace: cronjob.namespace().clone(),
            ..Default::default()
        },
        spec: cronjob.spec.job_template.spec.clone(),
        status: None,
    };
    const SCHEDULED_TIME_ANNOTATION: &str = "batch.tutorial.kube.rs/scheduled-at";
    if let Some(job_template_meta) = &cronjob.spec.job_template.metadata {
        if let Some(annotations) = &job_template_meta.annotations {
            for (key, value) in annotations {
                job.annotations_mut().insert(key.clone(), value.clone());
            }
        }

        if let Some(labels) = &job_template_meta.labels {
            for (key, value) in labels {
                job.labels_mut().insert(key.clone(), value.clone());
            }
        }
    }
    job.annotations_mut().insert(
        SCHEDULED_TIME_ANNOTATION.to_string(),
        DateTime::<Local>::from(scheduled_time).to_rfc3339(),
    );

    // TODO not sure about this. It is meant to set controller ownership
    let controller_ref = cronjob.controller_owner_ref(&());
    job.metadata
        .owner_references
        .get_or_insert(controller_ref.into_iter().collect());

    Ok(job)
}
