extern crate rsmorphy;

use rsmorphy::prelude::*;

pub mod util;

use util::print_row_parsed;


fn table(morph: &MorphAnalyzer, s: &str) {
    for (i, parsed) in morph.parse(s).into_iter().enumerate() {
        let enc = parsed.lex.encoded();
        let (s, dec) = Lex::decode(&enc).unwrap();
        let decoded = Parsed { lex: dec, score: parsed.score }; //dec.score()
        assert_eq!(s, "");
        print_row_parsed(morph, i, &parsed);
        print_row_parsed(morph, i, &decoded);
        println!();
    }
}


fn main() {
    let morph_ru = rsmorphy::load_test_morph_ru();

//    table(&morph_ru, "яблоко");
//    table(&morph_ru, "хлеб");
//    table(&morph_ru, "рыба");
    table(&morph_ru, "стали");
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
