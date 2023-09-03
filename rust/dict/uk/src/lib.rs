pub mod meta {
    include!(concat!(env!("OUT_DIR"), "/meta.rs"));
}

pub use crate::meta::DICT_PATH;
