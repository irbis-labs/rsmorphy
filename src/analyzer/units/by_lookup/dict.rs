use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Dictionary, Word};
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::stack::StackSource;
use dawg::HH;



#[derive(Default, Debug, Clone)]
pub struct DictionaryAnalyzer {}


impl Analyzer for DictionaryAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("DictionaryAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let para_data = morph.dict.words.similar_items(word_lower, &morph.dict.char_substitutes);
        trace!(r#" para_data="{:?}" "#, para_data);

        // `fixed_word` is a word with proper substitute (e.g. ё) letters
        for (fixed_word, parses) in para_data {

            for HH(para_id, idx) in parses {
                let container = Dictionary {
                    word_lower: Word {
                        word: fixed_word.clone(),
                        is_known: true,
                    },
                    para_id: para_id,
                    idx: idx,
                };
                result.push(Parsed {
                    lex: Lex::from_stack(morph, StackSource::from(container)),
                    score: Score::Real(1.0),
                });
            }
        }
    }
}
