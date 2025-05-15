use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Kubernetes error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Kubernetes request error: {0}")]
    KubeRequestError(#[from] kube_core::request::Error),
    #[error("Kube dynamic parsing error: {0}")]
    KubeParsingError(#[from] kube_core::dynamic::ParseDynamicObjectError),
    #[error("Label missing for {0}")]
    MissingLabelError(String),
    #[error("Status missing for {0}")]
    MissingStatusError(String),
    #[error("UID missing for {0}")]
    MissingUIDError(String),
    #[error("Object \"{0}\" not found for deletion")]
    InvalidDeleteError(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
