use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=data/");

    let dict_dir = Path::new("./data").canonicalize()?;

    let out_dir = Path::new(&env::var("OUT_DIR").unwrap()).canonicalize()?;

    {
        let data_path = &out_dir.join("data.rs");

        #[cfg(feature = "include_static")]
        {
            use lib::collections::dictionary::DictionaryView;
            use lib::data_format::binpack::mem_pack;
            use lib::data_format::pymorphy2::LanguageCode;
            use lib::data_format::pymorphy2::Pymorphy2Adapter;
            use lib::data_format::LoadError;

            #[path = "src/config/char_substitutes.rs"]
            mod char_substitutes;
            #[path = "src/config/initial_letters.rs"]
            mod initial_letters;

            use char_substitutes::CHAR_SUBSTITUTES;
            use initial_letters::INITIAL_LETTERS;

            let loader = Pymorphy2Adapter::open(&dict_dir).unwrap();
            let meta = &loader.meta();
            if meta.language_code != LanguageCode::Ru {
                Err::<(), _>(LoadError::InvalidLanguage(meta.language_code)).unwrap();
            }
            let dict = loader.load(INITIAL_LETTERS, CHAR_SUBSTITUTES).unwrap();
            let dict_view = DictionaryView::from_storage(&dict);
            let bin = match env::var("CARGO_CFG_TARGET_ENDIAN").unwrap().as_str() {
                "little" => mem_pack::pack::<byteorder::LittleEndian>(dict_view),
                "big" => mem_pack::pack::<byteorder::BigEndian>(dict_view),
                endianness => panic!("Unknown endianness: {endianness:?}"),
            };

            let bin_path = &out_dir.join("dict.bin");
            let mut f = File::create(bin_path)?;
            f.write_all(&bin)?;

            let mut f = File::create(data_path)?;

            writeln!(
                f,
                r###"
use lib::binutils::Align64;

pub const DICT_STATIC: Align64<[u8; {}]> = Align64(*include_bytes!({:?}));
        "###,
                bin.len(),
                bin_path
            )?;
        }

        #[cfg(not(feature = "include_static"))]
        {
            let _f = File::create(data_path)?;
        }
    }

    {
        let meta_path = &out_dir.join("meta.rs");
        let mut f = File::create(meta_path)?;

        writeln!(
            f,
            r###"
    /// Path to the dictionary directory in the build environment.
    /// Usefull for tests, examples, benchmarks, and local development.
    pub const DEV_DICT_PATH: &str = {:?};
    "###,
            dict_dir
        )?;
    }

    Ok(())
}
