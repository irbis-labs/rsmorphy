#[cfg(not(feature = "lang-uk"))]
fn main() {
    println!("This example is only available for Ukrainian language");
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
    let mut buffer = morph.make_buffer();

    lib_dev::input_loop(|word| table(&mut buffer, word));

    fn table<L: Lang>(buffer: &mut ParseBuffers<L>, word: &str) {
        let morph = buffer.morph();
        for (i, parse) in buffer.parse(word).drain().enumerate() {
            println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &parse));
        }
    }
}
