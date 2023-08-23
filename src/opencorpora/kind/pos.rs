use self::PartOfSpeech as POS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartOfSpeech {
    /// имя существительное
    Noun,
    /// имя прилагательное (полное)
    AdjectiveFull,
    /// имя прилагательное (краткое)
    AdjectiveShort,
    /// компаратив
    Comparative,
    /// глагол (личная форма)
    Verb,
    /// глагол (инфинитив)
    Infinitive,
    /// причастие (полное)
    ParticipleFull,
    /// причастие (краткое)
    ParticipleShort,
    /// деепричастие
    Gerund,
    /// числительное
    Number,
    /// наречие
    Adverb,
    /// местоимение-существительное
    Pronoun,
    /// предикатив
    Predicative,
    /// предлог
    Preposition,
    /// союз
    Conjunction,
    /// частица
    Particle,
    /// междометие
    Interjection,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         NOUN
        |ADJF
        |ADJS
        |COMP
        |VERB
        |INFN
        |PRTF
        |PRTS
        |GRND
        |NUMR
        |ADVB
        |NPRO
        |PRED
        |PREP
        |CONJ
        |PRCL
        |INTJ
    )
"
);

impl PartOfSpeech {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "NOUN" => Some(POS::Noun),
                "ADJF" => Some(POS::AdjectiveFull),
                "ADJS" => Some(POS::AdjectiveShort),
                "COMP" => Some(POS::Comparative),
                "VERB" => Some(POS::Verb),
                "INFN" => Some(POS::Infinitive),
                "PRTF" => Some(POS::ParticipleFull),
                "PRTS" => Some(POS::ParticipleShort),
                "GRND" => Some(POS::Gerund),
                "NUMR" => Some(POS::Number),
                "ADVB" => Some(POS::Adverb),
                "NPRO" => Some(POS::Pronoun),
                "PRED" => Some(POS::Predicative),
                "PREP" => Some(POS::Preposition),
                "CONJ" => Some(POS::Conjunction),
                "PRCL" => Some(POS::Particle),
                "INTJ" => Some(POS::Interjection),
                _ => None,
            })
    }

    pub fn is_productive(self) -> bool {
        match self {
            POS::Conjunction
            | POS::Number
            | POS::Pronoun
            | POS::Predicative
            | POS::Preposition
            | POS::Particle
            | POS::Interjection => false,
            _ => true,
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            POS::Noun => "имя существительное",
            POS::AdjectiveFull => "имя прилагательное (полное)",
            POS::AdjectiveShort => "имя прилагательное (краткое)",
            POS::Comparative => "компаратив",
            POS::Verb => "глагол (личная форма)",
            POS::Infinitive => "глагол (инфинитив)",
            POS::ParticipleFull => "причастие (полное)",
            POS::ParticipleShort => "причастие (краткое)",
            POS::Gerund => "деепричастие",
            POS::Number => "числительное",
            POS::Adverb => "наречие",
            POS::Pronoun => "местоимение-существительное",
            POS::Predicative => "предикатив",
            POS::Preposition => "предлог",
            POS::Conjunction => "союз",
            POS::Particle => "частица",
            POS::Interjection => "междометие",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(
            Some(PartOfSpeech::Noun),
            PartOfSpeech::try_from_fmt_int("NOUN")
        );
        assert_eq!(
            Some(PartOfSpeech::Noun),
            PartOfSpeech::try_from_fmt_int("NOUN,anim,masc,Fixd,Abbr sing,nomn")
        );
        assert_eq!(None, PartOfSpeech::try_from_fmt_int("UNKN"));
    }
}
