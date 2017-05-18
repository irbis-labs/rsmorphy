use super::error::DecodeError;


pub fn take_1_char(s: &str) -> Result<(&str, char), DecodeError> {
    let ch = s.chars().next().ok_or(DecodeError::UnexpectedEnd)?;
    Ok((&s[ch.len_utf8() .. ], ch))
}


pub fn take_str_while_char<'s, P>(s: &'s str, mut predicate: P) -> Result<(&'s str, &'s str), DecodeError>
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


pub fn take_str_while_char_is<'s>(s: &'s str, chr: char) -> Result<(&'s str, &'s str), DecodeError> {
    take_str_while_char(s, |ch| ch == chr )
}

pub fn take_str_until_char_is<'s>(s: &'s str, chr: char) -> Result<(&'s str, &'s str), DecodeError> {
    take_str_until(s, |ch| ch == chr )
}
