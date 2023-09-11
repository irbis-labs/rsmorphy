use std::marker::PhantomData;

use crate::analyzer::common::HyphenAdverb;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::WordStruct;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::model::gram::GramRegistry;
use crate::model::tag::TagReg;
use crate::GramSet;

const RU_HA_PREFIX: &str = "по-";
const RU_HA_PREFIX_LEN: usize = 3;
const RU_MIN_WORD_LEN: usize = 5;
const RU_MIN_REMNANT_LEN: usize = RU_MIN_WORD_LEN - RU_HA_PREFIX_LEN;
const SCORE_MULTIPLIER: f64 = 0.7;

/// Detect adverbs that starts with "по-".
///
/// Example: по-западному

#[derive(Debug, Clone)]
pub struct HyphenAdverbAnalyzer<L: Lang> {
    pub tag: TagReg,
    pub score_multiplier: f64,
    pub adjf_sing_datv: GramSet,
    _lang: PhantomData<L>,
}

impl<L: Lang> HyphenAdverbAnalyzer<L> {
    pub fn new(grammemes: GramRegistry<'_>, score_multiplier: Option<f64>) -> Self {
        let adjf_sing_datv = ["ADJF", "sing", "datv"]
            .into_iter()
            .map(|gram_int| grammemes.find_gram(gram_int).unwrap())
            .collect();
        HyphenAdverbAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "ADVB"),
            score_multiplier: score_multiplier.unwrap_or(SCORE_MULTIPLIER),
            adjf_sing_datv,
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("HyphenAdverbAnalyzer::parse(); {ctx:?}");

        let morph = ctx.morph();
        let word = ctx.word();
        if !word.starts_with(RU_HA_PREFIX) {
            return;
        }
        let remnant = &word[RU_HA_PREFIX.len()..];
        if remnant.chars().count() < RU_MIN_REMNANT_LEN {
            return;
        }

        morph
            .make_buffer()
            .prepare(remnant)
            .parse()
            // Currently, we don't use the score, so we can skip the estimation.
            // .estimate()
            .drain()
            .filter(|parse| {
                morph
                    .get_tag(&parse.lex)
                    .grammemes
                    .is_superset(self.adjf_sing_datv)
            })
            .for_each(|parse| {
                let word = WordStruct::new(ctx.word_lower().clone(), parse.lex.is_known());
                let lex = Lex::from_base(HyphenAdverb::new(word));
                ctx.push(Parse::new(lex, Score::Fake(self.score_multiplier)));
            });
    }
}
