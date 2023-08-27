use std::collections::BTreeSet;
use std::iter::FromIterator;

use dict_ru;
use rsmorphy::prelude::*;
use rsmorphy::util::format::ExplainLex;

pub mod util;

use crate::util::input_loop;

fn print_lexeme(morph: &MorphAnalyzer, lex: &Lex) {
    for (i, lex) in lex.iter_lexeme(morph).enumerate() {
        println!(r#"{:2}. {}"#, i + 1, ExplainLex(morph, &lex));
    }
}

fn list(morph: &MorphAnalyzer, s: &str) {
    let set = BTreeSet::from_iter(
        morph
            .parse(s)
            .into_iter()
            .map(|parsed| parsed.lex.get_lemma(morph).encoded()),
    );
    for (i, id) in set.into_iter().enumerate() {
        let lemma = Lex::from_id(id).unwrap();
        println!("  === {}. {} ===", i + 1, lemma.get_word());
        print_lexeme(morph, &lemma);
    }
}

fn main() {
    let _ = env_logger::init();

    let morph_ru = MorphAnalyzer::from_file(dict_ru::DICT_PATH);

    input_loop(|word| list(&morph_ru, word));
}
