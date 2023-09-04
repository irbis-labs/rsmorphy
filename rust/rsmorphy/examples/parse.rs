use rsmorphy::prelude::*;
use rsmorphy::util::format::ExplainParse;

pub mod util;

use crate::util::input_loop;

fn table<L: Lang>(buffer: &mut ParseBuffers<L>, word: &str) {
    let morph = buffer.morph();
    for (i, parse) in buffer.parse(word).drain().enumerate() {
        println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &parse));
    }
}

fn main() {
    let _ = env_logger::init();

    let morph = Russian::load_pymorphy2_dict(dict_ru::DICT_PATH).unwrap();
    // let morph = Ukrainian::load_pymorphy2_dict(dict_uk::DICT_PATH).unwrap();

    let mut buffer = morph.make_buffer();

    input_loop(|word| table(&mut buffer, word))
}
