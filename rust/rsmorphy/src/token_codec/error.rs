use std::num::ParseFloatError;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DecodeError {
    /// A decoder expects more
    #[error("Unexpected end of input")]
    UnexpectedEnd,
    /// An unexpected type code reached
    // TODO #[error("Unexpected type code: {0}")]
    #[error("Unexpected type code")]
    UnknownPartType,
    /// An input doesnt match the current pattern
    #[error("Input doesnt match the current pattern")]
    DoesntMatch,
    /// The number decoder failed to parse an integer
    #[error("The number decoder failed to parse an integer: {0}")]
    ParseIntError(ParseIntError),
    /// The number decoder failed to parse a float
    #[error("The number decoder failed to parse a float: {0}")]
    ParseFloatError(ParseFloatError),
    /// The string decoder failed to validate parameter
    #[error("The string decoder failed to validate parameter")]
    BadData,
}

impl From<ParseIntError> for DecodeError {
    fn from(e: ParseIntError) -> Self {
        DecodeError::ParseIntError(e)
    }
}

impl From<ParseFloatError> for DecodeError {
    fn from(e: ParseFloatError) -> Self {
        DecodeError::ParseFloatError(e)
    }
}
