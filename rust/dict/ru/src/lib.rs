pub mod config;

pub mod meta {
    include!(concat!(env!("OUT_DIR"), "/meta.rs"));
}

pub mod data {
    include!(concat!(env!("OUT_DIR"), "/data.rs"));
}

pub use crate::meta::DEV_DICT_PATH;
