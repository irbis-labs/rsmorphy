use self::PartOfSpeech as POS;
use crate::model::gram::GramId;

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

impl PartOfSpeech {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::PartOfSpeech::*;

        Some(match id.as_str() {
            "NOUN" => Noun,
            "ADJF" => AdjectiveFull,
            "ADJS" => AdjectiveShort,
            "COMP" => Comparative,
            "VERB" => Verb,
            "INFN" => Infinitive,
            "PRTF" => ParticipleFull,
            "PRTS" => ParticipleShort,
            "GRND" => Gerund,
            "NUMR" => Number,
            "ADVB" => Adverb,
            "NPRO" => Pronoun,
            "PRED" => Predicative,
            "PREP" => Preposition,
            "CONJ" => Conjunction,
            "PRCL" => Particle,
            "INTJ" => Interjection,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            POS::Noun => "NOUN",
            POS::AdjectiveFull => "ADJF",
            POS::AdjectiveShort => "ADJS",
            POS::Comparative => "COMP",
            POS::Verb => "VERB",
            POS::Infinitive => "INFN",
            POS::ParticipleFull => "PRTF",
            POS::ParticipleShort => "PRTS",
            POS::Gerund => "GRND",
            POS::Number => "NUMR",
            POS::Adverb => "ADVB",
            POS::Pronoun => "NPRO",
            POS::Predicative => "PRED",
            POS::Preposition => "PREP",
            POS::Conjunction => "CONJ",
            POS::Particle => "PRCL",
            POS::Interjection => "INTJ",
        }
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
            PartOfSpeech::from_gram_id(GramId::from_str("NOUN").unwrap())
        );
        assert_eq!(
            None,
            PartOfSpeech::from_gram_id(GramId::from_str("UNKN").unwrap())
        );
    }
}
