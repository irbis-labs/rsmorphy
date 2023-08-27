use crate::analyzer::context::ParseContext;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::DictionaryWord;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Score;
use crate::analyzer::model::WordStruct;
use crate::analyzer::unit::AnalyzerUnit;
use crate::dawg::HH;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::dictionary::ParadigmIndex;

const DICT_SCORE: Score = Score::Real(1.0);

#[derive(Debug, Clone, Copy)]
pub struct DictionaryAnalyzer {}

impl DictionaryAnalyzer {
    pub(in crate::analyzer) fn new() -> Self {
        DictionaryAnalyzer {}
    }
}

impl AnalyzerUnit for DictionaryAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("DictionaryAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let morph = ctx.morph();

        let para_data = morph
            .dict()
            .words
            .similar_items(ctx.word_lower(), &morph.dict().char_substitutes);
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" para_data="{:?}" "#, para_data);

        // `fixed_word` is a word with proper substitute (e.g. ё) letters
        for (fixed_word, parses) in para_data {
            for HH(para_id, idx) in parses {
                let para_id = ParadigmId::new(para_id);
                let idx = ParadigmIndex::new(idx);

                let word_lower = WordStruct::known(fixed_word.clone());
                let container = DictionaryWord::new(word_lower, para_id, idx);
                let lex = Lex::from_stack(StackSource::from(container));
                ctx.push(Parsed::new(lex, DICT_SCORE));
            }
        }
    }
}
