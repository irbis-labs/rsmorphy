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


regex!(TAG_RE, r"(?x)
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
");


impl PartOfSpeach {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "NOUN"  => Some(PartOfSpeach::Noun),
                "ADJF"  => Some(PartOfSpeach::Adjf),
                "ADJS"  => Some(PartOfSpeach::Adjs),
                "COMP"  => Some(PartOfSpeach::Comp),
                "VERB"  => Some(PartOfSpeach::Verb),
                "INFN"  => Some(PartOfSpeach::Infn),
                "PRTF"  => Some(PartOfSpeach::Prtf),
                "PRTS"  => Some(PartOfSpeach::Prts),
                "GRND"  => Some(PartOfSpeach::Grnd),
                "NUMR"  => Some(PartOfSpeach::Numr),
                "ADVB"  => Some(PartOfSpeach::Advb),
                "NPRO"  => Some(PartOfSpeach::Npro),
                "PRED"  => Some(PartOfSpeach::Pred),
                "PREP"  => Some(PartOfSpeach::Prep),
                "CONJ"  => Some(PartOfSpeach::Conj),
                "PRCL"  => Some(PartOfSpeach::Prcl),
                "INTJ"  => Some(PartOfSpeach::Intj),
                _       => None,
            },
            None => None,
        }
    }

    pub fn is_productive(&self) -> bool {
        match *self {
            PartOfSpeach::Numr |
            PartOfSpeach::Npro |
            PartOfSpeach::Pred |
            PartOfSpeach::Prep |
            PartOfSpeach::Conj |
            PartOfSpeach::Prcl |
            PartOfSpeach::Intj => false,
            _ => true
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(Some(PartOfSpeach::Noun), PartOfSpeach::try_from_str("NOUN"));
        assert_eq!(Some(PartOfSpeach::Noun), PartOfSpeach::try_from_str("NOUN,anim,masc,Fixd,Abbr sing,nomn"));
        assert_eq!(None, PartOfSpeach::try_from_str("UNKN"));
    }
}
