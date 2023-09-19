#[cfg(not(feature = "lang-uk"))]
fn main() {
    println!("This example is only available for Russian language");
}

#[cfg(feature = "lang-uk")]
fn main() {
    use rsmorphy::prelude::*;
    use rsmorphy::util::format::ExplainParse;

    let _ = lib_dev::init_logger();

    #[cfg(not(feature = "dict_load-include_static"))]
    let morph = Ukrainian::load_pymorphy2_dict(dict_uk::DEV_DICT_PATH).unwrap();

    #[cfg(feature = "dict_load-include_static")]
    let morph = Ukrainian::new_static();
    let words = [
        "яблуко",
        "хліб",
        "риба",
        "того",
        "зерно",
        "зернах",
        "Є",
        "БГ",
        "БГ-с",
        "дивись-но",
        "людина-пароплав",
        "інтернет-магазин",
        "псевдокішку",
        "байткод",
        "бутявкати",
        "бутявкає",
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
            assert_eq!(parse, decoded);
            println!();
        }
    }
}
