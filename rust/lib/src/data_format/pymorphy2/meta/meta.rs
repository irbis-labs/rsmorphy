use smallvec::SmallVec;
use smart_string::SmartString;

use super::CompileOptions;
use super::GramtabFormats;
use super::LanguageCode;
use super::PTWOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pymorphy2Meta {
    pub language_code: LanguageCode,
    pub format_version: SmartString,
    pub pymorphy2_version: SmartString,
    pub compiled_at: SmartString,
    pub source: SmartString,
    pub source_version: SmartString,
    pub source_revision: SmartString,
    pub source_lexemes_count: u32,
    pub source_links_count: u32,
    pub gramtab_length: u32,
    pub gramtab_formats: GramtabFormats,
    pub paradigms_length: u32,
    pub suffixes_length: u32,
    pub words_dawg_length: u32,
    pub compile_options: CompileOptions,
    pub prediction_suffixes_dawg_lengths: SmallVec<[u32; 8]>,
    pub p_t_w: Option<PTWOptions>,
    pub corpus_revision: Option<SmartString>,
}

mod serde_impl {
    use std::fmt;

    use serde::de::SeqAccess;
    use serde::Deserialize;
    use serde::Deserializer;
    use smallvec::SmallVec;

    use super::*;

    #[derive(Debug, Default)]
    struct Helper {
        language_code: Option<LanguageCode>,
        format_version: Option<SmartString>,
        pymorphy2_version: Option<SmartString>,
        compiled_at: Option<SmartString>,
        source: Option<SmartString>,
        source_version: Option<SmartString>,
        source_revision: Option<SmartString>,
        source_lexemes_count: Option<u32>,
        source_links_count: Option<u32>,
        gramtab_length: Option<u32>,
        gramtab_formats: Option<GramtabFormats>,
        paradigms_length: Option<u32>,
        suffixes_length: Option<u32>,
        words_dawg_length: Option<u32>,
        compile_options: Option<CompileOptions>,
        prediction_suffixes_dawg_lengths: Option<SmallVec<[u32; 8]>>,
        p_t_w: Option<bool>,
        p_t_w_unique_words: Option<u32>,
        p_t_w_outcomes: Option<u32>,
        p_t_w_min_word_freq: Option<u32>,
        corpus_revision: Option<SmartString>,
    }

    #[derive(Debug)]
    #[cfg_attr(test, derive(PartialEq, Eq, Clone))]
    pub enum Field {
        LanguageCode(LanguageCode),
        FormatVersion(SmartString),
        Pymorphy2Version(SmartString),
        CompiledAt(SmartString),
        Source(SmartString),
        SourceVersion(SmartString),
        SourceRevision(SmartString),
        SourceLexemesCount(u32),
        SourceLinksCount(u32),
        GramtabLength(u32),
        GramtabFormats(GramtabFormats),
        ParadigmsLength(u32),
        SuffixesLength(u32),
        WordsDawgLength(u32),
        CompileOptions(CompileOptions),
        PredictionSuffixesDawgLengths(SmallVec<[u32; 8]>),
        PTW(bool),
        PTWUniqueWords(u32),
        PTWOutcomes(u32),
        PTWMinWordFreq(u32),
        CorpusRevision(SmartString),
    }

    impl Helper {
        fn finish(self) -> Result<Pymorphy2Meta, MissingField<'static>> {
            Ok(Pymorphy2Meta {
                language_code: check(self.language_code, "language_code")?,
                format_version: check(self.format_version, "format_version")?,
                pymorphy2_version: check(self.pymorphy2_version, "pymorphy2_version")?,
                compiled_at: check(self.compiled_at, "compiled_at")?,
                source: check(self.source, "source")?,
                source_version: check(self.source_version, "source_version")?,
                source_revision: check(self.source_revision, "source_revision")?,
                source_lexemes_count: check(self.source_lexemes_count, "source_lexemes_count")?,
                source_links_count: check(self.source_links_count, "source_links_count")?,
                gramtab_length: check(self.gramtab_length, "gramtab_length")?,
                gramtab_formats: check(self.gramtab_formats, "gramtab_formats")?,
                paradigms_length: check(self.paradigms_length, "paradigms_length")?,
                suffixes_length: check(self.suffixes_length, "suffixes_length")?,
                words_dawg_length: check(self.words_dawg_length, "words_dawg_length")?,
                compile_options: check(self.compile_options, "compile_options")?,
                prediction_suffixes_dawg_lengths: check(
                    self.prediction_suffixes_dawg_lengths,
                    "prediction_suffixes_dawg_lengths",
                )?,
                p_t_w: if self.p_t_w.unwrap_or(false) {
                    Some(PTWOptions {
                        unique_words: check(self.p_t_w_unique_words, "P(t|w)_unique_words")?,
                        outcomes: check(self.p_t_w_outcomes, "P(t|w)_outcomes")?,
                        min_word_freq: check(self.p_t_w_min_word_freq, "P(t|w)_min_word_freq")?,
                    })
                } else {
                    None
                },
                corpus_revision: self.corpus_revision,
            })
        }
    }

    impl<'de> Deserialize<'de> for Pymorphy2Meta {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::Error;
            use serde::de::Visitor;

            struct Pymorphy2MetaVisitor;

            impl<'de> Visitor<'de> for Pymorphy2MetaVisitor {
                type Value = Pymorphy2Meta;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct Pymorphy2Meta")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Pymorphy2Meta, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let mut helper = Helper::default();

                    while let Some(field) = seq.next_element::<Field>()? {
                        match field {
                            Field::LanguageCode(v) => helper.language_code = Some(v),
                            Field::FormatVersion(v) => helper.format_version = Some(v),
                            Field::Pymorphy2Version(v) => helper.pymorphy2_version = Some(v),
                            Field::CompiledAt(v) => helper.compiled_at = Some(v),
                            Field::Source(v) => helper.source = Some(v),
                            Field::SourceVersion(v) => helper.source_version = Some(v),
                            Field::SourceRevision(v) => helper.source_revision = Some(v),
                            Field::SourceLexemesCount(v) => helper.source_lexemes_count = Some(v),
                            Field::SourceLinksCount(v) => helper.source_links_count = Some(v),
                            Field::GramtabLength(v) => helper.gramtab_length = Some(v),
                            Field::GramtabFormats(v) => helper.gramtab_formats = Some(v),
                            Field::ParadigmsLength(v) => helper.paradigms_length = Some(v),
                            Field::SuffixesLength(v) => helper.suffixes_length = Some(v),
                            Field::WordsDawgLength(v) => helper.words_dawg_length = Some(v),
                            Field::CompileOptions(v) => helper.compile_options = Some(v),
                            Field::PredictionSuffixesDawgLengths(v) => {
                                helper.prediction_suffixes_dawg_lengths = Some(v)
                            }
                            Field::PTW(v) => helper.p_t_w = Some(v),
                            Field::PTWUniqueWords(v) => helper.p_t_w_unique_words = Some(v),
                            Field::PTWOutcomes(v) => helper.p_t_w_outcomes = Some(v),
                            Field::PTWMinWordFreq(v) => helper.p_t_w_min_word_freq = Some(v),
                            Field::CorpusRevision(v) => helper.corpus_revision = Some(v),
                        }
                    }

                    helper.finish().map_err(Error::custom)
                }
            }

            deserializer.deserialize_seq(Pymorphy2MetaVisitor)
        }
    }

    fn check<T>(value: Option<T>, name: &'static str) -> Result<T, MissingField> {
        value.ok_or_else(|| MissingField(name))
    }
    impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::Error;
            use serde::de::Visitor;

            struct FieldVisitor;

            // deserialize variant from pairs, e.g. ["language_code", "ru"] -> Field::LanguageCode("ru")
            impl<'de> Visitor<'de> for FieldVisitor {
                type Value = Field;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("key-value pair")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Field, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let key: SmartString = seq
                        .next_element()?
                        .ok_or_else(|| Error::custom("Field key is missing"))?;

                    fn value<T>(
                        value: Option<T>,
                        name: &str,
                        f: impl FnOnce(T) -> Field,
                    ) -> Result<Field, MissingValue> {
                        Ok(f(value.ok_or_else(|| MissingValue(name))?))
                    }

                    let key = key.as_ref();
                    Ok(match key {
                        "language_code" => value(seq.next_element()?, key, Field::LanguageCode),
                        "format_version" => value(seq.next_element()?, key, Field::FormatVersion),
                        "pymorphy2_version" => value(
                            seq.next_element()?,
                            "pymorphy2_version",
                            Field::Pymorphy2Version,
                        ),
                        "compiled_at" => value(seq.next_element()?, key, Field::CompiledAt),
                        "source" => value(seq.next_element()?, key, Field::Source),
                        "source_version" => value(seq.next_element()?, key, Field::SourceVersion),
                        "source_revision" => value(seq.next_element()?, key, Field::SourceRevision),
                        "source_lexemes_count" => {
                            value(seq.next_element()?, key, Field::SourceLexemesCount)
                        }
                        "source_links_count" => {
                            value(seq.next_element()?, key, Field::SourceLinksCount)
                        }
                        "gramtab_length" => value(seq.next_element()?, key, Field::GramtabLength),
                        "gramtab_formats" => value(seq.next_element()?, key, Field::GramtabFormats),
                        "paradigms_length" => {
                            value(seq.next_element()?, key, Field::ParadigmsLength)
                        }
                        "suffixes_length" => value(seq.next_element()?, key, Field::SuffixesLength),
                        "words_dawg_length" => {
                            value(seq.next_element()?, key, Field::WordsDawgLength)
                        }
                        "compile_options" | "build_options" | "prediction_options" => {
                            value(seq.next_element()?, key, Field::CompileOptions)
                        }
                        "prediction_suffixes_dawg_lengths" => value(
                            seq.next_element()?,
                            key,
                            Field::PredictionSuffixesDawgLengths,
                        ),
                        "P(t|w)" => value(seq.next_element()?, key, Field::PTW),
                        "P(t|w)_unique_words" => {
                            value(seq.next_element()?, key, Field::PTWUniqueWords)
                        }
                        "P(t|w)_outcomes" => value(seq.next_element()?, key, Field::PTWOutcomes),
                        "P(t|w)_min_word_freq" => {
                            value(seq.next_element()?, key, Field::PTWMinWordFreq)
                        }
                        "corpus_revision" => value(seq.next_element()?, key, Field::CorpusRevision),

                        _ => Err(Error::unknown_field(
                            key,
                            &[
                                "language_code",
                                "format_version",
                                "pymorphy2_version",
                                "compiled_at",
                                "source",
                                "source_version",
                                "source_revision",
                                "source_lexemes_count",
                                "source_links_count",
                                "gramtab_length",
                                "gramtab_formats",
                                "paradigms_length",
                                "suffixes_length",
                                "words_dawg_length",
                                "compile_options",
                                "build_options",
                                "prediction_suffixes_dawg_lengths",
                                "P(t|w)",
                                "P(t|w)_unique_words",
                                "P(t|w)_outcomes",
                                "P(t|w)_min_word_freq",
                                "corpus_revision",
                            ],
                        ))?,
                    }
                    .map_err(Error::custom)?)
                }
            }

            deserializer.deserialize_seq(FieldVisitor)
        }
    }

    pub struct MissingField<'a>(&'a str);
    impl fmt::Display for MissingField<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Field key {:?} is missing", self.0)
        }
    }

    pub struct MissingValue<'a>(&'a str);

    impl fmt::Display for MissingValue<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Field value {:?} is missing", self.0)
        }
    }

    #[cfg(test)]
    mod tests {
        use lib_dev::init_logger;

        use super::*;

        #[test]
        fn test_decode_field() {
            init_logger();

            let json = r#"["language_code", "ru"]"#;
            let field: serde_impl::Field = serde_json::from_str(json).unwrap();
            assert_eq!(field, serde_impl::Field::LanguageCode(LanguageCode::Ru));
        }

        #[test]
        fn test_decode_meta() {
            let json = r###"[
      [
        "language_code",
        "ru"
      ],
      [
        "format_version",
        "2.4"
      ],
      [
        "pymorphy2_version",
        "0.9.1"
      ],
      [
        "compiled_at",
        "2020-10-11T15:05:51.070345"
      ],
      [
        "source",
        "opencorpora.org"
      ],
      [
        "source_version",
        "0.92"
      ],
      [
        "source_revision",
        "417127"
      ],
      [
        "source_lexemes_count",
        391764
      ],
      [
        "source_links_count",
        258604
      ],
      [
        "gramtab_length",
        5532
      ],
      [
        "gramtab_formats",
        {
          "opencorpora-ext": "gramtab-opencorpora-ext.json",
          "opencorpora-int": "gramtab-opencorpora-int.json"
        }
      ],
      [
        "paradigms_length",
        3456
      ],
      [
        "suffixes_length",
        16311
      ],
      [
        "words_dawg_length",
        5140055
      ],
      [
        "compile_options",
        {
          "min_ending_freq": 2,
          "min_paradigm_popularity": 3,
          "max_suffix_length": 5,
          "paradigm_prefixes": [
            "",
            "по",
            "наи"
          ]
        }
      ],
      [
        "prediction_suffixes_dawg_lengths",
        [
          375311,
          1882,
          21
        ]
      ],
      [
        "P(t|w)",
        true
      ],
      [
        "P(t|w)_unique_words",
        45458
      ],
      [
        "P(t|w)_outcomes",
        518189
      ],
      [
        "P(t|w)_min_word_freq",
        1
      ],
      [
        "corpus_revision",
        "4579844"
      ]
    ]"###;
            let meta: Pymorphy2Meta = serde_json::from_str(json).unwrap();
            assert_eq!(
                meta,
                Pymorphy2Meta {
                    language_code: LanguageCode::Ru,
                    format_version: "2.4".into(),
                    pymorphy2_version: "0.9.1".into(),
                    compiled_at: "2020-10-11T15:05:51.070345".into(),
                    source: "opencorpora.org".into(),
                    source_version: "0.92".into(),
                    source_revision: "417127".into(),
                    source_lexemes_count: 391764,
                    source_links_count: 258604,
                    gramtab_length: 5532,
                    gramtab_formats: GramtabFormats {
                        opencorpora_ext: "gramtab-opencorpora-ext.json".into(),
                        opencorpora_int: "gramtab-opencorpora-int.json".into(),
                    },
                    paradigms_length: 3456,
                    suffixes_length: 16311,
                    words_dawg_length: 5140055,
                    compile_options: CompileOptions {
                        min_ending_freq: 2,
                        min_paradigm_popularity: 3,
                        max_suffix_length: 5,
                        paradigm_prefixes: ["".into(), "по".into(), "наи".into()]
                            .into_iter()
                            .collect(),
                    },
                    prediction_suffixes_dawg_lengths: [375311, 1882, 21].into_iter().collect(),
                    p_t_w: Some(PTWOptions {
                        unique_words: 45458,
                        outcomes: 518189,
                        min_word_freq: 1,
                    }),
                    corpus_revision: Some("4579844".into()),
                }
            );
        }
    }
}
