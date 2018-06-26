use analyzer::MorphAnalyzer;
use analyzer::units::abc::AnalyzerUnit;
use container::{Dictionary, WordStruct};
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::stack::StackSource;
use dawg::HH;


const DICT_SCORE: Score = Score::Real(1.0);

#[derive(Default, Debug, Clone)]
pub struct DictionaryAnalyzer {}

impl AnalyzerUnit for DictionaryAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("DictionaryAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let para_data = morph.dict.words.similar_items(word_lower, &morph.dict.char_substitutes);
        trace!(r#" para_data="{:?}" "#, para_data);

        // `fixed_word` is a word with proper substitute (e.g. ё) letters
        for (fixed_word, parses) in para_data {
            for HH(para_id, idx) in parses {
                let word_lower = WordStruct::new(fixed_word.clone(), false);
                let container = Dictionary::new(word_lower, para_id, idx);
                let lex = Lex::from_stack(morph, StackSource::from(container));
                result.push(Parsed::new(lex, DICT_SCORE));
            }
        }
    }
}
