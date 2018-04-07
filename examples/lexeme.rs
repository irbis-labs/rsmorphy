extern crate rsmorphy;

use rsmorphy::{
    prelude::*,
    rsmorphy_dict_ru,
};

pub mod util;

use util::print_row_lex;


fn print_lexeme(morph: &MorphAnalyzer, id: &str) {
    let lex = Lex::from_id(morph, id).unwrap();
    for (i, lex) in lex.iter_lexeme(morph).enumerate() {
        print_row_lex(morph, i, &lex);
    }
}


fn main() {
    let morph_ru = MorphAnalyzer::from_file(rsmorphy_dict_ru::DICT_PATH);

//    print_lexeme(&morph_ru, "ru:d:стали,388,4");
//    println!();
//    print_lexeme(&morph_ru, "ru:d:стали,d,1");
//    println!();
//    print_lexeme(&morph_ru, "ru:f:человек-пароход,21;ks:роход");
//    println!();
//    print_lexeme(&morph_ru, "ru:f:бутявкает,2cb,9;ks:вкает");
//    println!();
//    print_lexeme(&morph_ru, "ru:d:тявкать,f;up:бу");
//    println!();
//    print_lexeme(&morph_ru, "ru:d:кать,c8,6;up:бутяв");

    print_lexeme(&morph_ru, "ru:d:задач,5d,8");
}
