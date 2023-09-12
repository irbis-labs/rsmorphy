#[cfg(not(feature = "lang-ru"))]
fn main() {
    println!("This example is only available for Russian language");
}

#[cfg(feature = "lang-ru")]
fn main() {
    use rsmorphy::prelude::*;
    use rsmorphy::util::format::ExplainParse;

    let _ = lib_dev::init_logger();

    #[cfg(not(feature = "dict_load-include_static"))]
    let morph = Russian::load_pymorphy2_dict(dict_ru::DEV_DICT_PATH).unwrap();

    #[cfg(feature = "dict_load-include_static")]
    let morph = Russian::new_static();
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

    // Reusable buffer to avoid unnecessary allocations.

    let mut buffer = morph.make_buffer();
    for word in words {
        table(&mut buffer, word);
    }
    // Words, broken with wrong keyboard layout
    //    'ru:f:pешeние,4c;ks:ие'
    //    'ru:f:стeпeнь,3;ks:'
    //    'ru:f:болeе,b4;ks:'
    //    'ru:f:мoжeт,3;ks:'
    //    'ru:f:peзaть,3;ks:'
    //    table(&morph_ru, "задaч");

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
}
