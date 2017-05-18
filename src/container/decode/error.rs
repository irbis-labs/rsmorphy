use std::num::ParseIntError;
use std::num::ParseFloatError;


#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    UnexpectedEnd,
    UnknownPartType,
    DoesntMatch,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError)
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
