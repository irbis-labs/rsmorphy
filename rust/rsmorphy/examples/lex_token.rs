use dict_ru;
use rsmorphy::prelude::*;
use rsmorphy::util::format::ExplainParse;

pub mod util;

fn table(morph: &Russian, s: &str) {
    for (i, parse) in morph.parse(s).into_iter().enumerate() {
        let enc = morph.encode_token(&parse.lex).to_string();
        let dec = morph.decode_token(&enc).unwrap();
        let decoded = Parse {
            lex: dec,
            score: parse.score,
        };
        println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &parse));
        println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &decoded));
        println!();
    }
}

fn main() {
    let morph_ru = Russian::load_pymorphy2_dict(dict_ru::DICT_PATH).unwrap();

    // table(&morph_ru, "яблоко");
    //    table(&morph_ru, "хлеб");
    //    table(&morph_ru, "рыба");
    table(&morph_ru, "стали");
    // table(&morph_ru, "того");
    //    table(&morph_ru, "И");
    //    table(&morph_ru, "БГ");
    //    table(&morph_ru, "БГ-с");
    //    table(&morph_ru, "смотри-ка");
    //    table(&morph_ru, "человек-пароход");
    //    table(&morph_ru, "интернет-магазин");
    //    table(&morph_ru, "по-русски");
    //    table(&morph_ru, "по-западному");
    //    table(&morph_ru, "псевдокошку");
    //    table(&morph_ru, "байткод");
    //    table(&morph_ru, "бутявкать");
    //    table(&morph_ru, "бутявкает");
    //    table(&morph_ru, "pdf");
    //    table(&morph_ru, "42");
    //    table(&morph_ru, "3.14");
    //    table(&morph_ru, "XIII");
    //    table(&morph_ru, ", ");
    //    table(&morph_ru, ".");

    // Words, broken with wrong keyboard layout
    //    table(&morph_ru, "задaч");
    //    'ru:f:peзaть,3;ks:'
    //    'ru:f:мoжeт,3;ks:'
    //    'ru:f:болeе,b4;ks:'
    //    'ru:f:стeпeнь,3;ks:'
    //    'ru:f:pешeние,4c;ks:ие'
}
