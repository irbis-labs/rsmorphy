use rsmorphy::prelude::*;
use rsmorphy::util::format::ExplainParse;

pub mod util;

use crate::util::input_loop;

fn table<L: Lang>(morph: &L, s: &str) {
    for (i, parse) in morph.parse(s).into_iter().enumerate() {
        println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &parse));
    }
}

fn main() {
    let _ = env_logger::init();

    let morph = Russian::load_pymorphy2_dict(dict_ru::DICT_PATH).unwrap();
    // let morph = Ukrainian::load_pymorphy2_dict(dict_uk::DICT_PATH).unwrap();

    input_loop(|word| table(&morph, word))
}
