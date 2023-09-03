#[cfg(test)]
pub mod test {
    use crate::analyzer::Ukrainian;

    lazy_static::lazy_static! {
        pub static ref UK: Ukrainian = Ukrainian::load_pymorphy2_dict(dict_uk::DICT_PATH).unwrap();
    }
}
