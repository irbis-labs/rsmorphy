pub mod release {
    include!(concat!(env!("OUT_DIR"), "/release.rs"));
}

pub use release::DICT_PATH;
