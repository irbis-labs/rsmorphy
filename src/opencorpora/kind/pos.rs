#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartOfSpeach {
    /// имя существительное
    Noun,
    /// имя прилагательное (полное)
    Adjf,
    /// имя прилагательное (краткое)
    Adjs,
    /// компаратив
    Comp,
    /// глагол (личная форма)
    Verb,
    /// глагол (инфинитив)
    Infn,
    /// причастие (полное)
    Prtf,
    /// причастие (краткое)
    Prts,
    /// деепричастие
    Grnd,
    /// числительное
    Numr,
    /// наречие
    Advb,
    /// местоимение-существительное
    Npro,
    /// предикатив
    Pred,
    /// предлог
    Prep,
    /// союз
    Conj,
    /// частица
    Prcl,
    /// междометие
    Intj,
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

impl PartOfSpeach {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::PartOfSpeach::*;

        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "NOUN" => Some(Noun),
                "ADJF" => Some(Adjf),
                "ADJS" => Some(Adjs),
                "COMP" => Some(Comp),
                "VERB" => Some(Verb),
                "INFN" => Some(Infn),
                "PRTF" => Some(Prtf),
                "PRTS" => Some(Prts),
                "GRND" => Some(Grnd),
                "NUMR" => Some(Numr),
                "ADVB" => Some(Advb),
                "NPRO" => Some(Npro),
                "PRED" => Some(Pred),
                "PREP" => Some(Prep),
                "CONJ" => Some(Conj),
                "PRCL" => Some(Prcl),
                "INTJ" => Some(Intj),
                _ => None,
            })
    }

    pub fn is_productive(self) -> bool {
        use self::PartOfSpeach::*;

        match self {
            Conj | Numr | Npro | Pred | Prep | Prcl | Intj => false,
            _ => true,
        }
    }

    pub fn title_rus(self) -> &'static str {
        use self::PartOfSpeach::*;

        match self {
            Noun => "имя существительное",
            Adjf => "имя прилагательное (полное)",
            Adjs => "имя прилагательное (краткое)",
            Comp => "компаратив",
            Verb => "глагол (личная форма)",
            Infn => "глагол (инфинитив)",
            Prtf => "причастие (полное)",
            Prts => "причастие (краткое)",
            Grnd => "деепричастие",
            Numr => "числительное",
            Advb => "наречие",
            Npro => "местоимение-существительное",
            Pred => "предикатив",
            Prep => "предлог",
            Conj => "союз",
            Prcl => "частица",
            Intj => "междометие",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(Some(PartOfSpeach::Noun), PartOfSpeach::try_from_fmt_int("NOUN"));
        assert_eq!(
            Some(PartOfSpeach::Noun),
            PartOfSpeach::try_from_fmt_int("NOUN,anim,masc,Fixd,Abbr sing,nomn")
        );
        assert_eq!(None, PartOfSpeach::try_from_fmt_int("UNKN"));
    }
}
