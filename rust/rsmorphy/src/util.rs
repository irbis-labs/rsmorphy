#[cfg(test)]
pub mod test {
    use crate::MorphAnalyzer;

    lazy_static::lazy_static! {
        pub static ref RU: MorphAnalyzer = MorphAnalyzer::from_file(dict_ru::DICT_PATH).unwrap();
    }

    pub fn init_logger() {
        use std::sync::Once;

        static INIT: Once = Once::new();

        INIT.call_once(|| {
            env_logger::init();
        });
    }
}

pub mod format {
    use std::fmt;

    use crate::prelude::*;

    #[derive(Debug)]
    pub struct ExplainParsed<'s>(pub &'s MorphAnalyzer, pub &'s Parsed);

    impl<'s> fmt::Display for ExplainParsed<'s> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let morph = self.0;
            let &Parsed { ref lex, score } = self.1;
            let rf = match score {
                Score::Real(_) => "R",
                Score::Fake(_) => "F",
            };
            write!(
                f,
                "{rf} {score:7.5} :: {lex}",
                rf = rf,
                score = score.value(),
                lex = ExplainLex(morph, lex)
            )
        }
    }

    #[derive(Debug)]
    pub struct ExplainLex<'s>(pub &'s MorphAnalyzer, pub &'s Lex);

    impl<'s> fmt::Display for ExplainLex<'s> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let morph = self.0;
            let lex = self.1;
            write!(
                f,
                "{lex:20} — {norm:20} :: {enc:35} :: {tag:40}",
                lex = lex.get_word(),
                norm = lex.get_normal_form(morph),
                tag = lex.get_tag(morph).fmt_int,
                enc = lex.stack().encoded()
            )
        }
    }
}
