use thiserror::Error;

use super::GramId;

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Error)]
pub enum GramIdError {
    #[error("GramId length must be in range 1..=11, but got {0}")]
    InvalidLength(usize),
    #[error("GramId must be ASCII, but got {0:?}")]
    NotAscii([u8; GramId::MAX_LEN]),
}
