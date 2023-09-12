#[cfg(not(feature = "lang-uk"))]
fn main() {
    println!("This example is only available for Ukrainian language");
}

#[cfg(feature = "lang-uk")]
fn main() {
    use rsmorphy::prelude::*;

    let _ = lib_dev::init_logger();

    #[cfg(not(feature = "dict_load-include_static"))]
    let morph = Ukrainian::load_pymorphy2_dict(dict_uk::DEV_DICT_PATH).unwrap();
    #[cfg(feature = "dict_load-include_static")]
    let morph = Ukrainian::new_static();

    let apple = morph.decode_token("uk:d:яблуко,4d6").unwrap();
    let bread = morph.decode_token("uk:d:хліб,7").unwrap();
    let fish = morph.decode_token("uk:d:риба,c5").unwrap();

    let two = morph.decode_token("uk:d:два,657").unwrap();
    let five = morph.decode_token("uk:d:п'ять,4df").unwrap();

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
        " ::: {} {} не наїсися",
        1,
        apple
            .inflect(&morph, ablt)
            .unwrap()
            .find_plural(&morph, 1)
            .unwrap()
            .word_lower(),
    );
    println!(
        " ::: нагодував {} {} та {} {}",
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
