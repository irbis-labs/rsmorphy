use super::error::DecodeError;

pub fn follow_str<'s, 'm>(s: &'s str, m: &'m str) -> Result<&'s str, DecodeError> {
    if s.len() < m.len() {
        // FIXME maybe DoesntMatch?
        Err(DecodeError::UnexpectedEnd)?;
    }
    if s.as_bytes()
        .iter()
        .zip(m.as_bytes().iter())
        .all(|(a, b)| a == b)
    {
        Ok(&s[m.len()..])
    } else {
        Err(DecodeError::DoesntMatch)
    }
}
