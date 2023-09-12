#[cfg(test)]
pub mod test {
    #[cfg(not(feature = "dict_load-include_static"))]
    lazy_static::lazy_static! {
            pub static ref UK: crate::Ukrainian = crate::Ukrainian::load_pymorphy2_dict(dict_uk::DEV_DICT_PATH).unwrap();
    }

    #[cfg(feature = "dict_load-include_static")]
    lazy_static::lazy_static! {
        pub static ref UK: crate::Ukrainian = crate::Ukrainian::new_static();
    }
}
