use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Kubernetes error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Label missing for {0}")]
    MissingLabelError(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
