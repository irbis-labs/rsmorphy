use std::{
    str::FromStr,
    num::{
        ParseIntError,
        ParseFloatError
    }
};

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


pub fn (chr: char) {ch.is_digit(10)


pub fn take_1_char(s: &str) -> Result<(&str, char), DecodeError> {
    let ch = s.chars().next().ok_or(DecodeError::UnexpectedEnd)?;
    Ok((&s[ch.len_utf8() .. ], ch))
}


pub fn follow_str<'s, 'm>(s: &'s str, m: &'m str) -> Result<(&'s str, &'m str), DecodeError> {
    if s.len() < m.len() {
        Err(DecodeError::UnexpectedEnd)?;
    }
    if s.as_bytes().iter().zip(m.as_bytes().iter()).all(|(a, b)| a == b) {
        Ok((&s[m.len() .. ], m))
    } else {
        Err(DecodeError::DoesntMatch)
    }
}


pub fn take_str_while<'s, P>(s: &'s str, mut predicate: P) -> Result<(&'s str, &'s str), DecodeError>
    where
        P: FnMut(char) -> bool
{
    let mut pos = 0;
    for ch in s.chars() {
        if (predicate)(ch) {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok( (&s[pos ..], &s[.. pos]) )
}


pub fn take_str_until<'s, P>(s: &'s str, mut predicate: P) -> Result<(&'s str, &'s str), DecodeError>
    where
        P: FnMut(char) -> bool
{
    let mut pos = 0;
    for ch in s.chars() {
        if (predicate)(ch) {
            break;
        } else {
            pos += ch.len_utf8();
        }
    }
    Ok( (&s[pos ..], &s[.. pos]) )
}


pub fn take_str_while_char<'s>(s: &'s str, chr: char) -> Result<(&'s str, &'s str), DecodeError> {
    take_str_while(s, |ch| ch == chr )
}

pub fn take_str_until_char<'s>(s: &'s str, chr: char) -> Result<(&'s str, &'s str), DecodeError> {
    take_str_until(s, |ch| ch == chr )
}


pub fn map_parse_int<'s, 'm, T>( (s, m): (&'s str, &'m str) ) -> Result<(&'s str, T), DecodeError>
    where
        T: FromStr<Err = ParseIntError>
{
    Ok( (s, T::from_str(m)?) )
}


pub fn map_parse_float<'s, 'm, T>( (s, m): (&'s str, &'m str) ) -> Result<(&'s str, T), DecodeError>
    where
        T: FromStr<Err = ParseFloatError>
{
    Ok( (s, T::from_str(m)?) )
}

