use std::cmp::min;

use crate::analyzer::MorphAnalyzer;
use crate::container::ParseResult;
use crate::container::Parsed;
use crate::container::SeenSet;

pub fn add_parsed_if_not_seen(
    morph: &MorphAnalyzer,
    result: &mut ParseResult,
    seen_parses: &mut SeenSet,
    parsed: Parsed,
) {
    if seen_parses.insert(&parsed.lex.as_seen(morph)) {
        result.list.push(parsed);
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

    log::trace!("max_split: {}", max_split);
    let mut pos = 0;
    word.chars().take(max_split).map(move |ch| {
        pos += ch.len_utf8();
        (&word[..pos], &word[pos..])
    })
}

pub use self::profiler::DumbProfiler;

#[cfg(feature = "profile")]
mod profiler {
    use std::time::Instant;

    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub struct DumbProfiler {
        waypoint_start: Instant,
    }

    impl DumbProfiler {
        pub fn start() -> Self {
            let waypoint_start = Instant::now();
            DumbProfiler { waypoint_start }
        }

        pub fn waypoint(&mut self, s: &str) {
            let tm = ::std::time::Instant::now();
            eprintln!("{} :: {:?}", s, (tm - self.waypoint_start));
            self.waypoint_start = tm;
        }
    }
}

#[cfg(not(feature = "profile"))]
mod profiler {
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub struct DumbProfiler {}

    impl DumbProfiler {
        pub fn start() -> Self {
            DumbProfiler {}
        }

        pub fn waypoint(&mut self, _: &str) {}
    }
}

#[cfg(test)]
pub mod test {
    use crate::container::SeenSet;
    use crate::MorphAnalyzer;
    use crate::ParseResult;

    lazy_static::lazy_static! {
        pub static ref RU: MorphAnalyzer = MorphAnalyzer::from_file(dict_ru::DICT_PATH);
    }

    pub fn init_logger() {
        use std::sync::Once;

        static INIT: Once = Once::new();

        INIT.call_once(|| {
            env_logger::init();
        });
    }

    pub fn bootstrap_parse_ru(
        word: &str,
    ) -> (&'static MorphAnalyzer, ParseResult, &str, String, SeenSet) {
        (
            &RU,
            ParseResult::new(),
            word,
            word.to_lowercase(),
            SeenSet::default(),
        )
    }
}

pub mod format {
    use std::fmt;

    use crate::prelude::*;

    #[derive(Debug)]
    pub struct ExplainParsed<'s>(pub &'s MorphAnalyzer, pub &'s Parsed);

    impl<'s> fmt::Display for ExplainParsed<'s> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let morph = self.0;
            let &Parsed { ref lex, score } = self.1;
            let rf = match score {
                Score::Real(_) => "R",
                Score::Fake(_) => "F",
            };
            write!(
                f,
                "{rf} {score:7.5} :: {lex}",
                rf = rf,
                score = score.value(),
                lex = ExplainLex(morph, lex)
            )
        }
    }

    #[derive(Debug)]
    pub struct ExplainLex<'s>(pub &'s MorphAnalyzer, pub &'s Lex);

    impl<'s> fmt::Display for ExplainLex<'s> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let morph = self.0;
            let lex = self.1;
            write!(
                f,
                "{lex:20} — {norm:20} :: {enc:35} :: {tag:40}",
                lex = lex.get_word(),
                norm = lex.get_normal_form(morph),
                tag = lex.get_tag(morph).fmt_int,
                enc = lex.stack.encoded()
            )
        }
    }
}
