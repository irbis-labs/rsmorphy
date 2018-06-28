use std::cmp::min;
use std::mem;

use analyzer::MorphAnalyzer;
use container::{ParseResult, Parsed, SeenSet};

pub fn u16_from_slice(s: &[u8]) -> u16 {
    let mut buf = [0u8; 2];
    buf.copy_from_slice(&s[..2]);
    unsafe { mem::transmute::<[u8; 2], u16>(buf) }
}

pub fn u32_from_slice(s: &[u8]) -> u32 {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&s[..4]);
    unsafe { mem::transmute::<[u8; 4], u32>(buf) }
}

pub fn add_parsed_if_not_seen(
    morph: &MorphAnalyzer,
    result: &mut ParseResult,
    seen_parses: &mut SeenSet,
    parsed: Parsed,
) {
    if seen_parses.insert(&parsed.lex.as_seen(morph)) {
        result.push(parsed);
    }
}

/// Returns all splits of a `word` (taking into account `min_reminder` and `max_prefix_length`).
pub fn word_splits<'w: 'i, 'i, Rem, Pref>(
    word: &'w str,
    min_reminder: Rem,
    max_prefix_length: Pref,
) -> impl Iterator<Item = (&'w str, &'w str)> + 'i
where
    Rem: Into<Option<usize>>,
    Pref: Into<Option<usize>>,
{
    let min_reminder = min_reminder.into().unwrap_or(3);
    let max_prefix_length = max_prefix_length.into().unwrap_or(5);
    let char_len = word.chars().count();

    let max_split = if char_len > min_reminder {
        min(max_prefix_length, char_len - min_reminder)
    } else {
        0
    };

    trace!("max_split: {}", max_split);
    let mut pos = 0;
    word.chars().take(max_split).map(move |ch| {
        pos += ch.len_utf8();
        (&word[..pos], &word[pos..])
    })
}
