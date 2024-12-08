use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Kubernetes error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Chrono parse error: {0}")]
    ChronoParseError(#[from] k8s_openapi::chrono::ParseError),
    #[error("Spec missing from cronjob")]
    MissingSpec,
    #[error("Cronjob schedule parse error")]
    CronScheduleParseError(#[from] cron::error::Error),
    #[error("Too many missed starts")]
    TooManyMissedStarts,
    #[error("Missing earliest time")]
    MissingEarliestTime,
    #[error("System time error")]
    SystemTimeError(#[from] std::time::SystemTimeError),
}
