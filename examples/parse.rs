//extern crate rustyline;
extern crate rsmorphy;

use rsmorphy::{prelude::*, rsmorphy_dict_ru};

pub mod util;

use util::{input_loop, print_row_parsed};

fn table(morph: &MorphAnalyzer, s: &str) {
    for (i, parsed) in morph.parse(s).into_iter().enumerate() {
        print_row_parsed(morph, i, &parsed);
    }
}

fn main() {
    let morph_ru = MorphAnalyzer::from_file(rsmorphy_dict_ru::DICT_PATH);

    input_loop(|word| table(&morph_ru, word))
}
