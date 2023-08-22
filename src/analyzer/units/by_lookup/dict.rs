use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::stack::StackSource;
use crate::container::Dictionary;
use crate::container::Lex;
use crate::container::ParseResult;
use crate::container::Parsed;
use crate::container::Score;
use crate::container::SeenSet;
use crate::container::WordStruct;
use crate::dawg::HH;

const DICT_SCORE: Score = Score::Real(1.0);

#[derive(Default, Debug, Clone, Copy)]
pub struct DictionaryAnalyzer {}

impl AnalyzerUnit for DictionaryAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        #[cfg(trace_analyzer)]
        log::trace!("DictionaryAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let para_data = morph
            .dict()
            .words
            .similar_items(word_lower, &morph.dict().char_substitutes);
        #[cfg(trace_analyzer)]
        log::trace!(r#" para_data="{:?}" "#, para_data);

        // `fixed_word` is a word with proper substitute (e.g. ё) letters
        for (fixed_word, parses) in para_data {
            for HH(para_id, idx) in parses {
                let word_lower = WordStruct::known(fixed_word.clone());
                let container = Dictionary::new(word_lower, para_id, idx);
                let lex = Lex::from_stack(morph, StackSource::from(container));
                result.list.push(Parsed::new(lex, DICT_SCORE));
            }
        }
    }
}
