use std::cmp::min;

/// Returns all splits of a `word` (taking into account `min_reminder` and `max_prefix_length`).
pub fn word_splits<'w: 'i, 'i>(
    word: &'w str,
    min_reminder: usize,
    max_prefix_length: usize,
) -> impl Iterator<Item = (&'w str, &'w str)> + 'i {
    let char_len = word.chars().count();

    let max_split = if char_len > min_reminder {
        min(max_prefix_length, char_len - min_reminder)
    } else {
        0
    };

    #[cfg(feature = "trace_analyzer")]
    log::trace!("max_split: {}", max_split);
    let mut pos = 0;
    word.chars().take(max_split).map(move |ch| {
        pos += ch.len_utf8();
        (&word[..pos], &word[pos..])
    })
}
