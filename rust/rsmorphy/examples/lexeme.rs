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
    let mut lst = Vec::from_iter(morph.parse(s).into_iter().map(|parsed| {
        let lemma = parsed.lex.get_lemma(morph);
        (lemma.encoded(), lemma)
    }));
    lst.sort_by(|a, b| a.0.cmp(&b.0));
    lst.dedup_by(|a, b| a.0 == b.0);
    for (i, (_id, lemma)) in lst.into_iter().enumerate() {
        println!("  === {}. {} ===", i + 1, lemma.get_word());
        print_lexeme(morph, &lemma);
    }
}

fn main() {
    let _ = env_logger::init();

    let morph_ru = MorphAnalyzer::from_file(dict_ru::DICT_PATH).unwrap();

    input_loop(|word| list(&morph_ru, word));
}
