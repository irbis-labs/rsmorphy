use thiserror::Error;

#[allow(missing_copy_implementations)]
#[derive(Debug, Error)]
#[error("Can't join two different tags: at most one of them should be present")]
pub struct EitherError;
