use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON decode error: {0}")]
    JsonError(#[from] serde_json::Error),
}
