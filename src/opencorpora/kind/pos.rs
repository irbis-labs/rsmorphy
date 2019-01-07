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
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::PartOfSpeech::*;

        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "NOUN" => Some(Noun),
                "ADJF" => Some(AdjectiveFull),
                "ADJS" => Some(AdjectiveShort),
                "COMP" => Some(Comparative),
                "VERB" => Some(Verb),
                "INFN" => Some(Infinitive),
                "PRTF" => Some(ParticipleFull),
                "PRTS" => Some(ParticipleShort),
                "GRND" => Some(Gerund),
                "NUMR" => Some(Number),
                "ADVB" => Some(Adverb),
                "NPRO" => Some(Pronoun),
                "PRED" => Some(Predicative),
                "PREP" => Some(Preposition),
                "CONJ" => Some(Conjunction),
                "PRCL" => Some(Particle),
                "INTJ" => Some(Interjection),
                _ => None,
            })
    }

    pub fn is_productive(self) -> bool {
        use self::PartOfSpeech::*;

        match self {
            | Conjunction
            | Number
            | Pronoun
            | Predicative
            | Preposition
            | Particle
            | Interjection
            => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(Some(PartOfSpeech::Noun), PartOfSpeech::try_from_str("NOUN"));
        assert_eq!(
            Some(PartOfSpeech::Noun),
            PartOfSpeech::try_from_str("NOUN,anim,masc,Fixd,Abbr sing,nomn")
        );
        assert_eq!(None, PartOfSpeech::try_from_str("UNKN"));
    }
}
