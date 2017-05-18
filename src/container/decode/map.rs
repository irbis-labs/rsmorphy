use std::num::ParseIntError;
use std::num::ParseFloatError;
use std::str::FromStr;
use num::Num;

use super::error::DecodeError;


pub fn try_parse_int<'s, 'm, T>( (s, m): (&'s str, &'m str) ) -> Option<(&'s str, T)>
    where
        T: FromStr<Err = ParseIntError>
{
    parse_int((s, m)).ok()
}


pub fn parse_int<'s, 'm, T>((s, m): (&'s str, &'m str) ) -> Result<(&'s str, T), DecodeError>
    where
        T: FromStr<Err = ParseIntError>
{
    Ok( (s, T::from_str(m)?) )
}


pub fn parse_hex_int<'s, 'm, T>((s, m): (&'s str, &'m str) ) -> Result<(&'s str, T), DecodeError>
    where
        T: Num<FromStrRadixErr = ParseIntError>
{
    Ok( (s, T::from_str_radix(m, 16)?) )
}


pub fn parse_float<'s, 'm, T>((s, m): (&'s str, &'m str) ) -> Result<(&'s str, T), DecodeError>
    where
        T: FromStr<Err = ParseFloatError>
{
    Ok( (s, T::from_str(m)?) )
}

