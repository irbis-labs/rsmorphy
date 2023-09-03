#[cfg(test)]
pub mod test {
    use crate::analyzer::Russian;

    lazy_static::lazy_static! {
        pub static ref RU: Russian = Russian::load_pymorphy2_dict(dict_ru::DICT_PATH).unwrap();
    }
}
