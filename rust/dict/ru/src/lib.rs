pub mod release {
    include!(concat!(env!("OUT_DIR"), "/release.rs"));
}

pub use crate::release::DICT_PATH;
