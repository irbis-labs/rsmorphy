use std::cmp::min;

use analyzer::MorphAnalyzer;
use container::{ParseResult, Parsed, SeenSet};

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
