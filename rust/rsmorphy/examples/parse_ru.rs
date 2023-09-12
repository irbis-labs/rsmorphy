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
    let mut buffer = morph.make_buffer();

    lib_dev::input_loop(|word| table(&mut buffer, word));

    fn table<L: Lang>(buffer: &mut ParseBuffers<L>, word: &str) {
        let morph = buffer.morph();
        for (i, parse) in buffer.parse(word).drain().enumerate() {
            println!(r#"{:2}. {}"#, i + 1, ExplainParse::new(morph, &parse));
        }
    }
}
