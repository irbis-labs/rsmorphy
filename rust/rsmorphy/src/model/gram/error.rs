use inlinable_string::InlinableString;
use thiserror::Error;

use crate::model::gram::GramId;

#[derive(Debug, Clone, Error)]
pub enum GramRegistryError {
    #[error("Registry is full, overflowed at gram {0}")]
    RegistryIsFull(GramId),
    #[error("Parent Gram {0:?} not found")]
    ParentNotFound(InlinableString),
    #[error("{0}")]
    BadGramId(#[from] GramIdError),
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Error)]
pub enum GramIdError {
    #[error("GramId length must be in range 1..=11, but got {0}")]
    InvalidLength(usize),
    #[error("GramId must be ASCII, but got {0:?}")]
    NotAscii([u8; GramId::MAX_LEN]),
}
