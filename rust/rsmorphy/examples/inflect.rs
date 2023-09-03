use dict_ru;
use rsmorphy::prelude::*;

pub mod util;

fn main() {
    let _ = env_logger::init();

    {
        let rus = Russian::load_pymorphy2_dict(dict_ru::DICT_PATH).unwrap();

        let apple = rus.decode_token("ru:d:яблоко,247").unwrap();
        let bread = rus.decode_token("ru:d:хлеб,8d4").unwrap();
        let fish = rus.decode_token("ru:d:рыба,35").unwrap();

        let two = rus.decode_token("ru:d:два,51c").unwrap();
        let five = rus.decode_token("ru:d:пять,3a2").unwrap();

        let grms = &rus.dict().grammemes;
        let ablt = grms.find_gram("ablt").unwrap().as_set();

        println!(
            " ::: {} {} + {} {} = {} {}",
            1,
            apple.find_plural(&rus, 1).unwrap().word_lower(),
            4,
            apple.find_plural(&rus, 4).unwrap().word_lower(),
            5,
            apple.find_plural(&rus, 5).unwrap().word_lower(),
        );
        println!(
            " ::: {} {} - {} {} = {} {}",
            102,
            apple.find_plural(&rus, 102).unwrap().word_lower(),
            11,
            apple.find_plural(&rus, 11).unwrap().word_lower(),
            91,
            apple.find_plural(&rus, 91).unwrap().word_lower(),
        );
        println!(
            " ::: {} {} сыт не будешь",
            1,
            apple
                .inflect(&rus, ablt)
                .unwrap()
                .find_plural(&rus, 1)
                .unwrap()
                .word_lower(),
        );
        println!(
            " ::: накормил {} {} и {} {}",
            two.inflect(&rus, ablt).unwrap().word_lower(),
            bread
                .inflect(&rus, ablt)
                .unwrap()
                .find_plural(&rus, 2)
                .unwrap()
                .word_lower(),
            five.inflect(&rus, ablt).unwrap().word_lower(),
            fish.inflect(&rus, ablt)
                .unwrap()
                .find_plural(&rus, 5)
                .unwrap()
                .word_lower(),
        );
    }

    {
        let ukr = Ukrainian::load_pymorphy2_dict(dict_uk::DICT_PATH).unwrap();

        let apple = ukr.decode_token("uk:d:яблуко,4d6").unwrap();
        let bread = ukr.decode_token("uk:d:хліб,7").unwrap();
        let fish = ukr.decode_token("uk:d:риба,c5").unwrap();

        let two = ukr.decode_token("uk:d:два,657").unwrap();
        let five = ukr.decode_token("uk:d:п'ять,4df").unwrap();

        let grms = &ukr.dict().grammemes;
        let ablt = grms.find_gram("ablt").unwrap().as_set();

        println!(
            " ::: {} {} + {} {} = {} {}",
            1,
            apple.find_plural(&ukr, 1).unwrap().word_lower(),
            4,
            apple.find_plural(&ukr, 4).unwrap().word_lower(),
            5,
            apple.find_plural(&ukr, 5).unwrap().word_lower(),
        );
        println!(
            " ::: {} {} - {} {} = {} {}",
            102,
            apple.find_plural(&ukr, 102).unwrap().word_lower(),
            11,
            apple.find_plural(&ukr, 11).unwrap().word_lower(),
            91,
            apple.find_plural(&ukr, 91).unwrap().word_lower(),
        );
        println!(
            " ::: {} {} не наїсися",
            1,
            apple
                .inflect(&ukr, ablt)
                .unwrap()
                .find_plural(&ukr, 1)
                .unwrap()
                .word_lower(),
        );
        println!(
            " ::: нагодував {} {} та {} {}",
            two.inflect(&ukr, ablt).unwrap().word_lower(),
            bread
                .inflect(&ukr, ablt)
                .unwrap()
                .find_plural(&ukr, 2)
                .unwrap()
                .word_lower(),
            five.inflect(&ukr, ablt).unwrap().word_lower(),
            fish.inflect(&ukr, ablt)
                .unwrap()
                .find_plural(&ukr, 5)
                .unwrap()
                .word_lower(),
        );
    }
}
