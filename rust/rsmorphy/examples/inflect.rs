use dict_ru;
use rsmorphy::prelude::*;

pub mod util;

//use util::print_row_lex;

fn main() {
    let _ = env_logger::init();

    let morph_ru = MorphAnalyzer::from_file(dict_ru::DICT_PATH).unwrap();
    //let lex = Lex::from_id(&morph_ru, "ru:d:стали,388,4").unwrap();
    //print_row_lex(&morph_ru, 0, &lex.inflect(&morph_ru, &GrammemeSet::from_str("plur,ablt")).unwrap());
    //print_row_lex(&morph_ru, 1, &lex.inflect(&morph_ru, &GrammemeSet::from_str("femn,sing,ablt,V-ey")).unwrap());
    //println!();

    let apple = Lex::from_id("ru:d:яблоко,22c").unwrap();
    let bread = Lex::from_id("ru:d:хлеб,878").unwrap();
    let fish = Lex::from_id("ru:d:рыба,35").unwrap();

    let two = Lex::from_id("ru:d:два,4e0").unwrap();
    let five = Lex::from_id("ru:d:пять,379").unwrap();

    let grms = &morph_ru.dict().grammemes;
    let ablt_set = grms.find_gram("ablt").unwrap().as_set();

    println!(
        " ::: {} {} + {} {} = {} {}",
        1,
        apple.get_plural(&morph_ru, 1).unwrap(),
        4,
        apple.get_plural(&morph_ru, 4).unwrap(),
        5,
        apple.get_plural(&morph_ru, 5).unwrap(),
    );
    println!(
        " ::: {} {} - {} {} = {} {}",
        102,
        apple.get_plural(&morph_ru, 102).unwrap(),
        11,
        apple.get_plural(&morph_ru, 11).unwrap(),
        91,
        apple.get_plural(&morph_ru, 91).unwrap(),
    );
    println!(
        " ::: {} {} сыт не будешь",
        1,
        apple
            .inflect(&morph_ru, ablt_set)
            .unwrap()
            .get_plural(&morph_ru, 1)
            .unwrap(),
    );
    println!(
        " ::: накормил {} {} и {} {}",
        two.inflect(&morph_ru, ablt_set).unwrap(),
        bread
            .inflect(&morph_ru, ablt_set)
            .unwrap()
            .get_plural(&morph_ru, 2)
            .unwrap(),
        five.inflect(&morph_ru, ablt_set).unwrap(),
        fish.inflect(&morph_ru, ablt_set)
            .unwrap()
            .get_plural(&morph_ru, 5)
            .unwrap(),
    );
}
