#[cfg(not(feature = "lang-ru"))]
fn main() {
    println!("This example is only available for Russian language");
}

#[cfg(feature = "lang-ru")]
fn main() {
    use rsmorphy::prelude::*;

    let _ = lib_dev::init_logger();

    #[cfg(not(feature = "dict_load-include_static"))]
    let morph = Russian::load_pymorphy2_dict(dict_ru::DEV_DICT_PATH).unwrap();
    #[cfg(feature = "dict_load-include_static")]
    let morph = Russian::new_static();

    let apple = morph.decode_token("ru:d:яблоко,247").unwrap();
    let bread = morph.decode_token("ru:d:хлеб,8d4").unwrap();
    let fish = morph.decode_token("ru:d:рыба,35").unwrap();

    let two = morph.decode_token("ru:d:два,51c").unwrap();
    let five = morph.decode_token("ru:d:пять,3a2").unwrap();

    let grms = morph.dict().grammemes();
    let ablt = grms.find_gram("ablt").unwrap().as_set();

    println!(
        " ::: {} {} + {} {} = {} {}",
        1,
        apple.find_plural(&morph, 1).unwrap().word_lower(),
        4,
        apple.find_plural(&morph, 4).unwrap().word_lower(),
        5,
        apple.find_plural(&morph, 5).unwrap().word_lower(),
    );
    println!(
        " ::: {} {} - {} {} = {} {}",
        102,
        apple.find_plural(&morph, 102).unwrap().word_lower(),
        11,
        apple.find_plural(&morph, 11).unwrap().word_lower(),
        91,
        apple.find_plural(&morph, 91).unwrap().word_lower(),
    );
    println!(
        " ::: {} {} сыт не будешь",
        1,
        apple
            .inflect(&morph, ablt)
            .unwrap()
            .find_plural(&morph, 1)
            .unwrap()
            .word_lower(),
    );
    println!(
        " ::: накормил {} {} и {} {}",
        two.inflect(&morph, ablt).unwrap().word_lower(),
        bread
            .inflect(&morph, ablt)
            .unwrap()
            .find_plural(&morph, 2)
            .unwrap()
            .word_lower(),
        five.inflect(&morph, ablt).unwrap().word_lower(),
        fish.inflect(&morph, ablt)
            .unwrap()
            .find_plural(&morph, 5)
            .unwrap()
            .word_lower(),
    );
}
