use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    /// A decoder expects more
    UnexpectedEnd,
    /// An unexpected type code reached
    UnknownPartType,
    /// An input doesnt match the current pattern
    DoesntMatch,
    /// The number decoder failed to parse an integer
    ParseIntError(ParseIntError),
    /// The number decoder failed to parse a float
    ParseFloatError(ParseFloatError),
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
