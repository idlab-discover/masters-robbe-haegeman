use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Get request failed resulting in Kubernetes error: {0}")]
    KubeGetError(#[from] kube::Error),
    #[error("Label missing for {0}")]
    MissingLabelError(String),
}
