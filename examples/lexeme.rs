extern crate rsmorphy;

use std::collections::BTreeSet;
use std::iter::FromIterator;

use dict_ru;
use rsmorphy::prelude::*;

pub mod util;

use crate::util::{input_loop, print_row_lex};

fn print_lexeme(morph: &MorphAnalyzer, lex: &Lex) {
    for (i, lex) in lex.iter_lexeme(morph).enumerate() {
        print_row_lex(morph, i, &lex);
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
        let lemma = Lex::from_id(morph, id).unwrap();
        println!("  === {}. {} ===", i + 1, lemma.get_word());
        print_lexeme(morph, &lemma);
    }
}

fn main() {
    let morph_ru = MorphAnalyzer::from_file(dict_ru::DICT_PATH);

    input_loop(|word| list(&morph_ru, word));
}
