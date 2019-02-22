use dict_ru;
use rsmorphy::prelude::*;

pub mod util;

//use util::print_row_lex;

fn main() {
    let morph_ru = MorphAnalyzer::from_file(dict_ru::DICT_PATH);

    //let lex = Lex::from_id(&morph_ru, "ru:d:стали,388,4").unwrap();
    //print_row_lex(&morph_ru, 0, &lex.inflect(&morph_ru, &GrammemeSet::from_str("plur,ablt")).unwrap());
    //print_row_lex(&morph_ru, 1, &lex.inflect(&morph_ru, &GrammemeSet::from_str("femn,sing,ablt,V-ey")).unwrap());
    //println!();

    let apple = Lex::from_id(&morph_ru, "ru:d:яблоко,22c").unwrap();
    let bread = Lex::from_id(&morph_ru, "ru:d:хлеб,878").unwrap();
    // let fish = Lex::from_id(&morph_ru, "ru:d:рыба,35").unwrap();

    let ablt_set = GrammemeSet::parse_fmt_int("ablt");

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
            .inflect(&morph_ru, &ablt_set)
            .unwrap()
            .get_plural(&morph_ru, 1)
            .unwrap(),
    );
    println!(
        " ::: накормил {} {}",
        2,
        bread
            .inflect(&morph_ru, &ablt_set)
            .unwrap()
            .get_plural(&morph_ru, 2)
            .unwrap(),
    );
    // FIXME рыбою => рыбами
    //println!(
    //    " ::: накормил {} {} и {} {}",
    //    2, bread.inflect(&morph_ru, &ablt_set).unwrap().get_plural(&morph_ru, 2).unwrap(),
    //    5, fish.inflect(&morph_ru, &ablt_set).unwrap().get_plural(&morph_ru, 5).unwrap(),
    //);
}
