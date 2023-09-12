#[cfg(test)]
pub mod test {
    #[cfg(not(feature = "dict_load-include_static"))]
    lazy_static::lazy_static! {
            pub static ref RU: crate::Russian = crate::Russian::load_pymorphy2_dict(dict_ru::DEV_DICT_PATH).unwrap();
    }

    #[cfg(feature = "dict_load-include_static")]
    lazy_static::lazy_static! {
        pub static ref RU: crate::Russian = crate::Russian::new_static();
    }
}
