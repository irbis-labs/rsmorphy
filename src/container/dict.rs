use std::{borrow::Cow, fmt};

use crate::{
    analyzer::MorphAnalyzer,
    container::{
        abc::*,
        decode::*,
        paradigm::{ParadigmId, ParadigmIndex},
        stack::StackSource,
        Lex, Score, WordStruct,
    },
    opencorpora::{paradigm::ParadigmEntry, tag::OpencorporaTagReg},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary {
    word_lower: WordStruct,
    para_id: ParadigmId,
    idx: ParadigmIndex,
}

impl Dictionary {
    pub fn new<ID, IDX>(word_lower: WordStruct, para_id: ID, idx: IDX) -> Self
    where
        ID: Into<ParadigmId>,
        IDX: Into<ParadigmIndex>,
    {
        let para_id = para_id.into();
        let idx = idx.into();
        Dictionary {
            word_lower,
            para_id,
            idx,
        }
    }

    pub fn word_lower(&self) -> &WordStruct {
        &self.word_lower
    }

    pub fn para_id(&self) -> ParadigmId {
        self.para_id
    }

    pub fn idx(&self) -> ParadigmIndex {
        self.idx
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        let paradigm = morph.dict.get_paradigm(self.para_id);
        let stem = morph
            .dict
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
                let prefix = &morph.dict.paradigm_prefixes[prefix_id as usize];
                let suffix = &morph.dict.suffixes[suffix_id as usize];
                let word = WordStruct::new(
                    format!("{}{}{}", prefix, stem, suffix),
                    self.word_lower.is_known(),
                );
                Lex::from_stack(
                    morph,
                    StackSource::from(Dictionary::new(word, self.para_id, idx)),
                )
            },
        )
    }
}

impl Source for Dictionary {
    fn score(&self) -> Score {
        Score::Real(1.0)
    }

    fn is_lemma(&self) -> bool {
        self.idx.is_first()
    }

    fn is_known(&self) -> bool {
        self.word_lower.is_known()
    }

    fn get_word(&self) -> Cow<str> {
        Cow::from(self.word_lower.word())
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        morph
            .dict
            .build_normal_form(self.para_id, self.idx, self.word_lower.word())
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        morph.dict.get_tag(self.para_id, self.idx)
    }

    fn try_get_para_id(&self) -> Option<ParadigmId> {
        Some(self.para_id)
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}", self.word_lower.word())
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        morph
            .dict
            .write_normal_form(f, self.para_id, self.idx, self.word_lower.word())
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}

impl MorphySerde for Dictionary {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}:", if self.word_lower.is_known() { "d" } else { "f" })?;
        write!(f, "{},{:x}", self.word_lower.word(), self.para_id.value())?;
        if !self.idx.is_first() {
            write!(f, ",{:x}", self.idx.value())?;
        }
        Ok(())
    }

    /**
        ```
        use rsmorphy::container::Dictionary;
        use rsmorphy::container::WordStruct;
        use rsmorphy::container::abc::*;

        assert_eq!(
            Dictionary::decode(r"d:сталь,d"),
            Ok(("", Dictionary::new(WordStruct::new("сталь", true), 0xd_u16, 0_u16)))
        );

        assert_eq!(
            Dictionary::decode(r"d:стали,d,1"),
            Ok(("", Dictionary::new(WordStruct::new("стали", true), 0xd_u16, 1_u16)))
        );

        assert_eq!(
            Dictionary::decode(r"f:бутявкает,2cb,9"),
            Ok(("", Dictionary::new(WordStruct::new("бутявкает", false), 0x2cb_u16, 9_u16)))
        );

        assert_eq!(
            Dictionary::decode(r"d:стали"),
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
            Dictionary::new(WordStruct::new(word, is_known), para_id, idx),
        ))
    }
}
