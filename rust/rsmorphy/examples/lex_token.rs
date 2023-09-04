use dict_ru;
use rsmorphy::prelude::*;
use rsmorphy::util::format::ExplainParse;

pub mod util;

fn table<L: Lang>(buffer: &mut ParseBuffers<L>, word: &str) {
    let morph = buffer.morph();
    for (i, parse) in buffer.parse(word).drain().enumerate() {
        println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &parse));
        let enc = morph.encode_token(&parse.lex).to_string();
        let dec = morph.decode_token(&enc).unwrap();
        let decoded = Parse {
            lex: dec,
            score: parse.score,
        };
        println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &decoded));
        println!();
    }
}

fn main() {
    let morph_ru = Russian::load_pymorphy2_dict(dict_ru::DICT_PATH).unwrap();
    let mut buffer_ru = morph_ru.make_buffer();
    let words = [
        "яблоко",
        "хлеб",
        "рыба",
        "стали",
        "того",
        "И",
        "БГ",
        "БГ-с",
        "смотри-ка",
        "человек-пароход",
        "интернет-магазин",
        "по-русски",
        "по-западному",
        "псевдокошку",
        "байткод",
        "бутявкать",
        "бутявкает",
        "pdf",
        "42",
        "3.14",
        "XIII",
        ", ",
        ".",
    ];

    for word in words {
        table(&mut buffer_ru, word);
    }

    // Words, broken with wrong keyboard layout
    //    table(&morph_ru, "задaч");
    //    'ru:f:peзaть,3;ks:'
    //    'ru:f:мoжeт,3;ks:'
    //    'ru:f:болeе,b4;ks:'
    //    'ru:f:стeпeнь,3;ks:'
    //    'ru:f:pешeние,4c;ks:ие'
}
