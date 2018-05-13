use std::borrow::Cow;
use std::fmt;

use analyzer::MorphAnalyzer;
use container::Lex;
use container::Score;
use container::Word;
use container::abc::*;
use container::stack::StackSource;
use opencorpora::paradigm::ParadigmEntry;
use opencorpora::tag::OpencorporaTagReg;

use container::decode::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary {
    pub word_lower: Word,
    pub para_id: u16,
    pub idx: u16,
}


impl Source for Dictionary {
    fn score(&self) -> Score {
        Score::Real(1.0)
    }

    fn is_lemma(&self) -> bool {
        self.idx == 0
    }

    fn is_known(&self) -> bool {
        self.word_lower.is_known
    }

    fn get_word(&self) -> Cow<str> {
        Cow::from(self.word_lower.word.as_ref())
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        morph.dict.build_normal_form(self.para_id, self.idx, self.word_lower.word.as_str())
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        morph.dict.build_tag_info(self.para_id, self.idx)
    }

    fn try_get_para_id(&self) -> Option<u16> {
        Some(self.para_id)
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}", self.word_lower.word)
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        morph.dict.write_normal_form(f, self.para_id, self.idx, self.word_lower.word.as_str())
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}


impl Dictionary {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        let paradigm = &morph.dict.paradigms[self.para_id as usize];
        let stem = morph.dict.build_stem(self.para_id, self.idx, self.word_lower.word.as_str());
        paradigm.iter().enumerate().map(move |(idx, &ParadigmEntry {prefix_id, suffix_id, ..})| {
            let prefix = &morph.dict.paradigm_prefixes[prefix_id as usize];
            let suffix = &morph.dict.suffixes[suffix_id as usize];
            Lex::from_stack(morph, StackSource::from(Dictionary {
                word_lower: Word {
                    word: format!("{}{}{}", prefix, stem, suffix),
                    is_known: self.word_lower.is_known
                },
                para_id: self.para_id,
                idx: idx as u16
            }))
        })
    }
}


impl MorphySerde for Dictionary {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}:", if self.word_lower.is_known { "d" } else { "f" })?;
        write!(f, "{},{:x}", self.word_lower.word, self.para_id)?;
        if self.idx != 0 {
            write!(f, ",{:x}", self.idx)?;
        }
        Ok(())
    }

    /**
        ```
        use rsmorphy::container::Dictionary;
        use rsmorphy::container::Word;
        use rsmorphy::container::abc::*;

        assert_eq!(
            Dictionary::decode(r"d:сталь,d"),
            Ok(("", Dictionary {
                word_lower: Word {
                    word: "сталь".into(),
                    is_known: true
                },
                para_id: 0xd,
                idx: 0
            }))
        );

        assert_eq!(
            Dictionary::decode(r"d:стали,d,1"),
            Ok(("", Dictionary {
                word_lower: Word {
                    word: "стали".into(),
                    is_known: true
                },
                para_id: 0xd,
                idx: 1
            }))
        );

        assert_eq!(
            Dictionary::decode(r"f:бутявкает,2cb,9"),
            Ok(("", Dictionary {
                word_lower: Word {
                    word: "бутявкает".into(),
                    is_known: false
                },
                para_id: 0x2cb,
                idx: 9
            }))
        );

        assert_eq!(
            Dictionary::decode(r"d:стали"),
            Err(DecodeError::UnexpectedEnd)
        );
        ```
    */
    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, is_known) = follow_str(s, "d").map(|s| (s, true))
            .or_else(|_| follow_str(s, "f").map(|s| (s, false)))
            .map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ',')?;
        let (s, para_id) = take_str_while_char(follow_str(s, ",")?, is_hex_digit)
            .and_then(parse_hex_int)?;
        let (s, idx) = follow_str(s, ",").ok()
            .map(|s| take_str_while_char(s, is_hex_digit).and_then(parse_hex_int))
            .unwrap_or_else(|| Ok((s, 0)))?;

        let word_lower = Word::new(word, is_known);
        Ok( (s, Dictionary { word_lower, para_id, idx }) )
    }
}
