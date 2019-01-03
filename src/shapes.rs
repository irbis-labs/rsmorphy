use roman;
use uc::UnicodeCategories;

#[inline]
pub fn is_ascii_alpha(ch: u8) -> bool {
    let ch = ch | 0x20;
    ch >= b'a' && ch <= b'z'
}

/**
    Return True if all token letters are latin and there is at
    least one latin letter in the token:

    ```
    use rsmorphy::shapes::is_latin;

    assert!(is_latin("foo"));
    assert!(is_latin("123-FOO"));

    assert!(!is_latin("123"));
    assert!(!is_latin(":)"));
    assert!(!is_latin(""));
    ```
*/
pub fn is_latin(token: &str) -> bool {
    token.is_ascii() && token.as_bytes().iter().cloned().any(is_ascii_alpha)
}

/**
    Return True if a word contains only spaces and punctuation marks
    and there is at least one punctuation mark:

    ```
    use rsmorphy::shapes::is_punctuation;

    assert!(is_punctuation(", "));
    assert!(is_punctuation("..!"));

    assert!(!is_punctuation("x"));
    assert!(!is_punctuation(" "));
    assert!(!is_punctuation(""));
    ```
*/
pub fn is_punctuation(token: &str) -> bool {
    !token.is_empty()
        && token.chars().any(|ch| ch.is_punctuation())
        && token
            .chars()
            .all(|ch| ch.is_punctuation() || ch.is_whitespace())
}

/**
    Return True if token looks like a Roman number:

    ```
    use rsmorphy::shapes::is_roman_number;

    assert!(is_roman_number("II"));
    assert!(is_roman_number("IX"));

    assert!(!is_roman_number("XIIIII"));
    assert!(!is_roman_number(""));
    ```
*/
pub fn is_roman_number(token: &str) -> bool {
    roman::from(token).is_some()
}
