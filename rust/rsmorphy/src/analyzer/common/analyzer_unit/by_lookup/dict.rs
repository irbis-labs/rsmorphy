use std::marker::PhantomData;

use crate::analyzer::common::DictionaryWord;
use crate::analyzer::common::Lex;
use crate::analyzer::common::Parse;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Score;
use crate::analyzer::common::WordStruct;
use crate::analyzer::Lang;
use crate::model::dictionary::WordValue;

const SCORE: Score = Score::Real(1.0);

#[derive(Debug, Clone, Copy)]
pub struct DictionaryAnalyzer<L: Lang> {
    score: Score,
    _lang: PhantomData<L>,
}

impl<L: Lang> DictionaryAnalyzer<L> {
    pub fn new(score: Option<Score>) -> Self {
        DictionaryAnalyzer {
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("DictionaryAnalyzer::parse(); {ctx:?}");

        let morph = ctx.morph();

        let para_data = morph
            .dict()
            .words
            .similar_items(ctx.word_lower(), &morph.dict().char_substitutes)
            .unwrap();
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" para_data="{:?}" "#, para_data);

        // `fixed_word` is a word with proper substitute (e.g. ё) letters
        for (fixed_word, parses) in para_data.into_iter().rev() {
            for WordValue { para_id, idx } in parses {
                let word_lower = WordStruct::known(fixed_word.clone());
                let lex = Lex::from_base(DictionaryWord::new(word_lower, para_id, idx));
                ctx.push(Parse::new(lex, self.score));
            }
        }
    }
}
