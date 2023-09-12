#[cfg(not(feature = "lang-ru"))]
fn main() {
    println!("This example is only available for Russian language");
}

#[cfg(feature = "lang-ru")]
fn main() {
    use rsmorphy::prelude::*;
    use rsmorphy::util::format::ExplainLex;

    let _ = lib_dev::init_logger();

    #[cfg(not(feature = "dict_load-include_static"))]
    let morph = Russian::load_pymorphy2_dict(dict_ru::DEV_DICT_PATH).unwrap();
    #[cfg(feature = "dict_load-include_static")]
    let morph = Russian::new_static();

    let mut buffer = morph.make_buffer();

    lib_dev::input_loop(move |word| list(&mut buffer, word));

    fn print_lexeme<L: Lang>(morph: &L, lex: &Lex) {
        for (i, lex) in morph.iter_lexeme(lex).enumerate() {
            println!(r#"{:>2}. {}"#, i + 1, ExplainLex::new(morph, &lex));
        }
    }

    fn list<L: Lang>(buffer: &mut ParseBuffers<L>, word: &str) {
        let morph = buffer.morph();
        let mut lst = Vec::from_iter(buffer.parse(word).drain().map(|parse| {
            let lemma = morph.get_lemma(&parse.lex);
            (morph.encode_token(&lemma).to_short_str(), lemma)
        }));
        lst.sort_by(|a, b| a.0.cmp(&b.0));
        lst.dedup_by(|a, b| a.0 == b.0);
        for (i, (_id, lemma)) in lst.into_iter().enumerate() {
            println!("  === {}. {} ===", i + 1, lemma.make_word_lower());
            print_lexeme(morph, &lemma);
        }
    }
}
