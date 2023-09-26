use smart_string::SmartString;
use thiserror::Error;

use crate::types::gram::GramId;
use crate::types::gram::GramIdError;

#[derive(Debug, Clone, Error)]
pub enum GramRegistryError {
    #[error("Registry is full, overflowed at gram {0}")]
    RegistryIsFull(GramId),
    #[error("Parent Gram {0:?} not found")]
    ParentNotFound(SmartString),
    #[error("{0}")]
    BadGramId(#[from] GramIdError),
}
