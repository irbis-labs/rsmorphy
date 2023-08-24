use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::model::WordStruct;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmEntry;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::dictionary::ParadigmIndex;
use crate::opencorpora::tag::OpencorporaTagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct DictionaryWord {
    pub(in crate::analyzer) word_lower: WordStruct,
    pub(in crate::analyzer) para_id: ParadigmId,
    pub(in crate::analyzer) idx: ParadigmIndex,
}

impl DictionaryWord {
    // FIXME rename into `new_unchecked`
    #[inline(always)]
    pub fn new<ID, IDX>(word_lower: WordStruct, para_id: ID, idx: IDX) -> Self
    where
        ID: Into<ParadigmId>,
        IDX: Into<ParadigmIndex>,
    {
        let para_id = para_id.into();
        let idx = idx.into();
        DictionaryWord {
            word_lower,
            para_id,
            idx,
        }
    }

    #[inline(always)]
    pub fn word_lower(&self) -> &WordStruct {
        &self.word_lower
    }

    #[inline(always)]
    pub fn para_id(&self) -> ParadigmId {
        self.para_id
    }

    #[inline(always)]
    pub fn idx(&self) -> ParadigmIndex {
        self.idx
    }

    #[inline(always)]
    pub fn into_parts(self) -> (WordStruct, ParadigmId, ParadigmIndex) {
        (self.word_lower, self.para_id, self.idx)
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        let paradigm = morph.dict().para().get(self.para_id).expect("paradigm id");
        let stem = morph
            .dict()
            .get_stem(self.para_id, self.idx, self.word_lower.word());
        paradigm.iter().enumerate().map(
            move |(
                idx,
                &ParadigmEntry {
                    prefix_id,
                    suffix_id,
                    ..
                },
            )| {
                let prefix = &morph.dict().paradigm_prefixes[prefix_id as usize];
                let suffix = &morph.dict().suffixes[suffix_id as usize];
                let word = WordStruct::new(
                    format!("{}{}{}", prefix, stem, suffix),
                    self.word_lower.is_known(),
                );
                Lex::from_stack(
                    StackSource::from(DictionaryWord::new(word, self.para_id, idx)),
                )
            },
        )
    }
}

impl Source for DictionaryWord {
    #[inline(always)]
    fn score(&self) -> Score {
        Score::Real(1.0)
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        self.idx.is_first()
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        self.word_lower.is_known()
    }

    #[inline(always)]
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        morph.dict().get_tag(self.para_id, self.idx)
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        Some(self.para_id)
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(self.word_lower.word())
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        morph
            .dict()
            .write_normal_form(f, self.para_id, self.idx, self.word_lower.word())
    }

    #[inline(always)]
    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    #[inline(always)]
    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}

impl MorphySerde for DictionaryWord {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        let known = if self.word_lower.is_known() { "d" } else { "f" };
        let word = self.word_lower.word();
        let para_id = self.para_id.value();
        write!(f, "{known}:{word},{para_id:x}")?;
        if !self.idx.is_first() {
            write!(f, ",{:x}", self.idx.value())?;
        }
        Ok(())
    }

    /**
        ```
        use rsmorphy::analyzer::model::DictionaryWord;
        use rsmorphy::analyzer::model::WordStruct;
        use rsmorphy::analyzer::model::abc::*;

        assert_eq!(
            DictionaryWord::decode(r"d:сталь,d"),
            Ok(("", DictionaryWord::new(WordStruct::new("сталь", true), 0xd_u16, 0_u16)))
        );

        assert_eq!(
            DictionaryWord::decode(r"d:стали,d,1"),
            Ok(("", DictionaryWord::new(WordStruct::new("стали", true), 0xd_u16, 1_u16)))
        );

        assert_eq!(
            DictionaryWord::decode(r"f:бутявкает,2cb,9"),
            Ok(("", DictionaryWord::new(WordStruct::new("бутявкает", false), 0x2cb_u16, 9_u16)))
        );

        assert_eq!(
            DictionaryWord::decode(r"d:стали"),
            Err(DecodeError::UnexpectedEnd)
        );
        ```
    */
    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, is_known) = follow_str(s, "d")
            .map(|s| (s, true))
            .or_else(|_| follow_str(s, "f").map(|s| (s, false)))
            .map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ',')?;
        let (s, para_id) = take_str_while_char(follow_str(s, ",")?, is_hex_digit)
            .and_then(parse_hex_int::<u16>)?;
        let (s, idx) = follow_str(s, ",")
            .ok()
            .map(|s| take_str_while_char(s, is_hex_digit).and_then(parse_hex_int::<u16>))
            .unwrap_or_else(|| Ok((s, 0)))?;
        // TODO assert `word` is in the lower case
        Ok((
            s,
            DictionaryWord::new(WordStruct::new(word, is_known), para_id, idx),
        ))
    }
}
