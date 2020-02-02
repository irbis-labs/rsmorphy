use dict_ru;
use rsmorphy::prelude::*;
use rsmorphy::util::format::ExplainParsed;

pub mod util;

use crate::util::{input_loop};

fn table(morph: &MorphAnalyzer, s: &str) {
    for (i, parsed) in morph.parse(s).into_iter().enumerate() {
        println!(r#"{:2}. {}"#, i + 1, ExplainParsed(morph, &parsed));
    }
}

fn main() {
    let morph_ru = MorphAnalyzer::from_file(dict_ru::DICT_PATH);

    input_loop(|word| table(&morph_ru, word))
}
