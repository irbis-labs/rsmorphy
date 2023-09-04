use std::fmt;
use std::path::Path;
use std::sync::Arc;

use inlinable_string::InlinableString;

use crate::analyzer::common::Affix;
use crate::analyzer::common::AffixKind;
use crate::analyzer::common::Base;
use crate::analyzer::common::DictionaryWord;
use crate::analyzer::common::HyphenAdverb;
use crate::analyzer::common::HyphenSeparatedParticle;
use crate::analyzer::common::Initial;
use crate::analyzer::common::Lex;
use crate::analyzer::common::Main;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::ShapeKind;
use crate::analyzer::common::Shaped;
use crate::analyzer::common::Unknown;
use crate::analyzer::common::WordStruct;
use crate::analyzer::uk::units::Units;
use crate::analyzer::Lang;
use crate::model::dictionary::data_format::pymorphy2::LanguageCode;
use crate::model::dictionary::data_format::Pymorphy2Adapter;
use crate::model::dictionary::LoadError;
use crate::model::dictionary::ParadigmId;
use crate::model::dictionary::ParadigmIndex;
use crate::model::kind::InitialKind;
use crate::model::tag::TagReg;
use crate::model::Dictionary;
use crate::token_codec::escape;
use crate::token_codec::follow_str;
use crate::token_codec::is_hex_digit;
use crate::token_codec::parse_hex_int;
use crate::token_codec::take_1_char;
use crate::token_codec::take_str_until_char_is;
use crate::token_codec::take_str_while_char;
use crate::token_codec::unescape;
use crate::token_codec::DecodeError;

#[derive(Debug, Clone)]
pub struct Ukrainian {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    dict: Dictionary,
    // estimator: SingleTagProbabilityEstimator,
    units: Units,
}

impl Lang for Ukrainian {
    fn dict(&self) -> &Dictionary {
        &self.inner.dict
    }

    fn parse_prepared<'m, 'w, 'b>(&self, context: &mut ParseContext<'m, 'w, 'b, Self>)
    where
        Self: Sized,
    {
        self.look_over(context);
    }

    fn estimate_prepared<'m, 'w, 'b>(&self, _context: &mut ParseContext<'m, 'w, 'b, Self>)
    where
        Self: Sized,
    {
        // context.apply_estimator(&self.inner.estimator);
    }

    fn get_tag(&self, lex: &Lex) -> &TagReg {
        use crate::analyzer::common::ShapeKind as SK;

        let units = &self.inner.units;
        match &lex.left.base {
            Base::Dictionary(dw) => self.dict().get_tag(dw.para_id, dw.idx),
            Base::HyphenAdverb(_) => unreachable!(),
            Base::Initial(initial) => &units.initial.tags[initial.tag_idx as usize].0,
            Base::Shaped(shaped) => match shaped.kind {
                SK::IntegerNumber => &units.number.tag_int,
                SK::FractionalNumber => &units.number.tag_real,
                SK::RomanNumber => &units.roman.tag,
                SK::Latin => &units.latin.tag,
                SK::Punctuation => &units.punct.tag,
            },
            Base::Unknown(_) => &units.unknown.tag,
        }
    }

    #[inline(always)]
    fn iter_lexeme<'s: 'i, 'l: 'i, 'i>(
        &'s self,
        lex: &'l Lex,
    ) -> Box<dyn Iterator<Item = Lex> + 'i> {
        // TODO take into account lex.right
        Box::new(
            self.iter_lexeme_main(&lex.left)
                .map(move |main: Main| Lex::new(main, None, lex.particle.clone())),
        )
    }

    fn write_token(&self, writer: &mut dyn fmt::Write, lex: &Lex) -> fmt::Result {
        writer.write_str("uk:")?;
        self.write_token_main(writer, &lex.left)?;
        if let Some(_right) = &lex.right {
            // writer.write_str("-")?;
            // self.encode_base(writer, right)?;
        }
        if let Some(particle) = &lex.particle {
            writer.write_str(";hp:")?;
            writer.write_str(&particle.particle)?;
        }
        Ok(())
    }

    fn decode_token(&self, token: &str) -> Result<Lex, DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_token({:?})", token);

        let s = follow_str(token, "uk:")?;
        let (s, left) = self.decode_main(s)?;
        let (s, right) = match follow_str(s, "-") {
            Ok(s) => {
                let (s, right) = self.decode_main(s)?;
                (s, Some(right))
            }
            Err(DecodeError::DoesntMatch | DecodeError::UnexpectedEnd) => (s, None),
            Err(e) => Err(e)?,
        };
        let (_s, particle) = match follow_str(s, ";") {
            Ok(s) => {
                let (s, particle) = self.decode_particle(s)?;
                (s, Some(particle))
            }
            Err(DecodeError::DoesntMatch | DecodeError::UnexpectedEnd) => (s, None),
            Err(e) => Err(e)?,
        };

        let res = Lex::new(left, right, particle);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_token({:?}) -> ({:?}, {:?})", token, _s, res);

        Ok(res)
    }
}

impl Ukrainian {
    /// Creates `MorphAnalyzer` with preloaded dict
    pub fn new(dict: Dictionary) -> Self {
        let units = Units::new(&dict.grammemes);
        let inner = Arc::new(Inner { dict, units });
        Ukrainian { inner }
    }

    /// Loads `Dictionary` from disk and creates `MorphAnalyzer`
    pub fn load_pymorphy2_dict(dict_dir: impl AsRef<Path>) -> Result<Self, LoadError> {
        let loader = Pymorphy2Adapter::open(dict_dir)?;
        let meta = &loader.meta();
        if meta.language_code != LanguageCode::Uk {
            return Err(LoadError::InvalidLanguage(meta.language_code));
        }
        let dict = loader.load()?;
        Ok(Ukrainian::new(dict))
    }

    fn look_over(&self, ctx: &mut ParseContext<Self>) {
        macro_rules! look_in (
                ($t: ident) => {{
                    self.inner.units.$t.parse(ctx);
                }};
                ($t: ident, terminal) => {{
                    self.inner.units.$t.parse(ctx);
                    if !ctx.parses().is_empty() { return };
                }}
            );

        look_in!(dictionary);
        look_in!(initial);
        look_in!(roman);
        look_in!(latin, terminal);

        look_in!(number, terminal);

        look_in!(punct, terminal);

        look_in!(hsp, terminal);

        look_in!(hword, terminal);

        look_in!(kp, terminal);

        look_in!(up);
        look_in!(ks, terminal);

        look_in!(unknown, terminal);

        unreachable!();
    }

    fn write_token_main(&self, writer: &mut dyn fmt::Write, main: &Main) -> fmt::Result {
        self.write_token_base(writer, &main.base)?;
        if let Some(affix) = &main.affix {
            let kind = match affix.kind {
                AffixKind::KnownPrefix => "kp",
                AffixKind::UnknownPrefix => "up",
                AffixKind::KnownSuffix => "ks",
            };
            let part = &affix.affix;
            write!(writer, ";{kind}:{part}")?;
        }
        Ok(())
    }

    fn write_token_base(&self, writer: &mut dyn fmt::Write, main: &Base) -> fmt::Result {
        match main {
            Base::Dictionary(dw) => {
                let known = if dw.word_lower.is_known { "d" } else { "f" };
                let word = &dw.word_lower.word;
                let para_id = dw.para_id.value();

                write!(writer, "{known}:{word},{para_id:x}")?;

                if !dw.idx.is_first() {
                    write!(writer, ",{:x}", dw.idx.value())?;
                }
            }
            Base::HyphenAdverb(ha) => {
                let word = &ha.word_lower.word;
                let kind = if ha.word_lower.is_known { "d" } else { "f" };
                write!(writer, "ha:{word},{kind}")?;
            }
            Base::Initial(initial) => {
                let kind = match initial.kind {
                    InitialKind::FirstName => "n",
                    InitialKind::Patronym => "p",
                };
                let gender = match (initial.tag_idx / 6) % 2 {
                    0 => "m",
                    1 => "f",
                    _ => unreachable!(),
                };
                let case = initial.tag_idx % 6;
                let letter = initial.letter;
                write!(writer, "i:{kind}{gender}{case},{letter}")?;
            }
            Base::Shaped(shaped) => {
                use ShapeKind as SK;

                let kind = match shaped.kind {
                    SK::IntegerNumber => "i",
                    SK::FractionalNumber => "f",
                    SK::RomanNumber => "r",
                    SK::Latin => "l",
                    SK::Punctuation => "p",
                };
                write!(writer, "s:{kind},")?;
                for ch in escape(&shaped.word) {
                    writer.write_str(ch)?;
                }
            }
            Base::Unknown(unknown) => {
                writer.write_str("u:")?;
                for ch in escape(&unknown.word_lower) {
                    writer.write_str(ch)?;
                }
            }
        }

        Ok(())
    }

    fn decode_particle<'t>(
        &self,
        token: &'t str,
    ) -> Result<(&'t str, HyphenSeparatedParticle), DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_particle({:?})", token);

        let s = follow_str(token, "hp").map_err(|e| match e {
            DecodeError::DoesntMatch => DecodeError::UnknownPartType,
            e => e,
        })?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;

        let res = HyphenSeparatedParticle::new(word);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_particle({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_main<'t>(&self, token: &'t str) -> Result<(&'t str, Main), DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_main({:?})", token);

        let (s, base) = self.decode_base(token)?;
        let mut res = Main::new(base, None);
        let s = if s.is_empty() {
            s
        } else {
            let s = follow_str(s, ";")?;
            let (s, affix) = self.decode_affix(s)?;
            res.affix = Some(affix);
            s
        };

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_main({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_affix<'t>(&self, token: &'t str) -> Result<(&'t str, Affix), DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_affix({:?})", token);

        let (s, kind) = follow_str(token, "ks")
            .map(|s| (s, AffixKind::KnownSuffix))
            .or_else(|_| follow_str(token, "kp").map(|s| (s, AffixKind::KnownPrefix)))
            .or_else(|_| follow_str(token, "up").map(|s| (s, AffixKind::UnknownPrefix)))
            .map_err(|e| match e {
                DecodeError::DoesntMatch => DecodeError::UnknownPartType,
                e => e,
            })?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;

        let res = Affix::new(word, kind);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_affix({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_base<'t>(&self, token: &'t str) -> Result<(&'t str, Base), DecodeError> {
        fn try_<T: Into<Base>>(
            v: Result<(&str, T), DecodeError>,
        ) -> Result<Option<(&str, Base)>, DecodeError> {
            Ok(match v {
                Err(DecodeError::UnknownPartType) => None,
                Err(e) => Err(e)?,
                Ok((s, v)) => Some((s, v.into())),
            })
        }

        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_base({:?})", token);

        let (s, res) = match try_(self.decode_dict(token))? {
            Some(v) => v,
            None => match try_(self.decode_hyphene_adverb(token))? {
                Some(v) => v,
                None => match try_(self.decode_initial(token))? {
                    Some(v) => v,
                    None => match try_(self.decode_shaped(token))? {
                        Some(v) => v,
                        None => match try_(self.decode_unknown(token))? {
                            Some(v) => v,
                            None => Err(DecodeError::UnknownPartType)?,
                        },
                    },
                },
            },
        };

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_base({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_dict<'t>(&self, token: &'t str) -> Result<(&'t str, DictionaryWord), DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_dict({:?})", token);

        let (s, is_known) = follow_str(token, "d")
            .map(|s| (s, true))
            .or_else(|_| follow_str(token, "f").map(|s| (s, false)))
            .map_err(|_| DecodeError::UnknownPartType)?;

        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ',')?;
        let (s, para_id) = take_str_while_char(follow_str(s, ",")?, is_hex_digit)
            .and_then(parse_hex_int::<u16>)?;
        let (s, idx) = follow_str(s, ",")
            .ok()
            .map(|s| take_str_while_char(s, is_hex_digit).and_then(parse_hex_int::<u16>))
            .unwrap_or_else(|| Ok((s, 0)))?;

        // TODO check paradigm reference.
        let para_id = ParadigmId::new(para_id);
        let idx = ParadigmIndex::new(idx);

        // TODO check whether the `word` is in the lower case.
        let res = DictionaryWord::new(WordStruct::new(word, is_known), para_id, idx);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_dict({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_hyphene_adverb<'t>(
        &self,
        token: &'t str,
    ) -> Result<(&'t str, HyphenAdverb), DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_hyphene_adverb({:?})", token);

        let s = follow_str(token, "ha").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ',')?;
        let (s, is_known) = follow_str(s, "d")
            .map(|s| (s, true))
            .or_else(|_| follow_str(s, "f").map(|s| (s, false)))
            .map_err(|_| DecodeError::UnknownPartType)?;

        // TODO check whether the `word` is in the lower case.
        let res = HyphenAdverb::new(WordStruct::new(word, is_known));

        #[cfg(feature = "trace_analyzer")]
        log::trace!(
            "< decode_hyphene_adverb({:?}) -> ({:?}, {:?})",
            token,
            s,
            res
        );

        Ok((s, res))
    }

    fn decode_initial<'t>(&self, token: &'t str) -> Result<(&'t str, Initial), DecodeError> {
        fn decode_tag_idx(kind: char, gender: char, case: char) -> Result<u8, DecodeError> {
            let kind = match kind {
                'n' => 0,
                'p' => 1,
                _ => Err(DecodeError::UnknownPartType)?,
            };
            let gender = match gender {
                'm' => 0,
                'f' => 1,
                _ => Err(DecodeError::UnknownPartType)?,
            };
            let case = match case {
                '0'..='5' => case as u8 - b'0',
                _ => Err(DecodeError::UnknownPartType)?,
            };
            Ok(kind * 12 + gender * 6 + case)
        }

        let s = follow_str(token, "i").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        let (s, gender) = take_1_char(s)?;
        let (s, case) = take_1_char(s)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;

        let tag_idx = decode_tag_idx(kind, gender, case)?;
        let kind = match kind {
            'n' => InitialKind::FirstName,
            'p' => InitialKind::Patronym,
            _ => Err(DecodeError::DoesntMatch)?,
        };
        let letter = self
            .inner
            .units
            .initial
            .letters
            .find(word)
            .ok_or(DecodeError::DoesntMatch)?;

        let res = Initial::new(letter, kind, tag_idx);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_initial({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_shaped<'t>(&self, token: &'t str) -> Result<(&'t str, Shaped), DecodeError> {
        use ShapeKind as SK;

        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_shaped({:?})", token);

        let s = follow_str(token, "s").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        // FIXME skip escaped ";"
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;
        let kind = match kind {
            'l' => SK::Latin,
            'f' => SK::FractionalNumber,
            'i' => SK::IntegerNumber,
            'p' => SK::Punctuation,
            'r' => SK::RomanNumber,
            _ => Err(DecodeError::UnknownPartType)?,
        };
        let word = unescape(word).collect();
        let res = Shaped { word, kind };

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_shaped({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn decode_unknown<'t>(&self, token: &'t str) -> Result<(&'t str, Unknown), DecodeError> {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("> decode_unknown({:?})", token);

        let s = follow_str(token, "u").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
        let word_lower = unescape(word).collect();
        let res = Unknown { word_lower };

        #[cfg(feature = "trace_analyzer")]
        log::trace!("< decode_unknown({:?}) -> ({:?}, {:?})", token, s, res);

        Ok((s, res))
    }

    fn iter_lexeme_main<'s: 'i, 'l: 'i, 'i>(
        &'s self,
        main: &'l Main,
    ) -> impl Iterator<Item = Main> + 'i {
        let is_known_suffix = main.affix.as_ref().map_or(false, Affix::is_known_suffix);
        let dict = &self.inner.dict;

        self.iter_lexeme_base(&main.base).map(move |base: Base| {
            let affix = if is_known_suffix {
                let Base::Dictionary(dw) = &base else {
                    panic!(
                        "iter_lexeme_main: expected Base::Dictionary, got {:?}",
                        base
                    )
                };
                let word = &dw.word_lower.word;
                let stem = dict.get_stem(dw.para_id, dw.idx, word);
                Some(Affix::known_suffix(&word[stem.len()..]))
            } else {
                main.affix.clone()
            };

            Main::new(base, affix)
        })
    }

    fn iter_lexeme_base<'s: 'i, 'l: 'i, 'i>(
        &'s self,
        base: &'l Base,
    ) -> Box<dyn Iterator<Item = Base> + 'i> {
        match base {
            Base::Dictionary(dw) => Box::new({
                let dict = &self.inner.dict;
                let stem = dict.get_stem(dw.para_id, dw.idx, &dw.word_lower.word);
                dict.iter_paradigm_info(dw.para_id).enumerate().map(
                    |(idx, (prefix, _tag, suffix))| {
                        let word = InlinableString::from(prefix) + stem + suffix;
                        let word = WordStruct::new(word, dw.word_lower.is_known);
                        let idx = ParadigmIndex::new(idx as u16);
                        Base::from(DictionaryWord::new(word, dw.para_id, idx))
                    },
                )
            }),
            Base::HyphenAdverb(ha) => Box::new([ha.clone()].into_iter().map(Base::from)),
            Base::Initial(initial) => Box::new({
                let len = self.inner.units.initial.tags.len() as u8;
                let range = match initial.kind {
                    InitialKind::FirstName => 0..len / 2,
                    InitialKind::Patronym => len / 2..len,
                };
                range.map(|tag_idx| {
                    Base::from(Initial {
                        tag_idx,
                        ..*initial
                    })
                })
            }),
            Base::Shaped(shaped) => Box::new([shaped.clone()].into_iter().map(Base::from)),
            Base::Unknown(unknown) => Box::new([unknown.clone()].into_iter().map(Base::from)),
        }
    }
}
