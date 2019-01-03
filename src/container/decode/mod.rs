pub mod error;
pub mod follow;
pub mod map;
pub mod predicate;
pub mod take;

pub use self::follow::*;
pub use self::map::*;
pub use self::predicate::*;
pub use self::take::*;

/**
```
use rsmorphy::container::decode::escape;

assert_eq!(escape(r"a,b").collect::<String>(),  String::from(r"a\,b"));
assert_eq!(escape(r"a;b").collect::<String>(),  String::from(r"a\;b"));
assert_eq!(escape(r"a:b").collect::<String>(),  String::from(r"a\:b"));
assert_eq!(escape(r"a\b").collect::<String>(),  String::from(r"a\\b"));
```
*/
pub fn escape<'s: 'i, 'i>(s: &'s str) -> impl Iterator<Item = &'s str> + 'i {
    s.split("").map(|ch| match ch {
        r"\" => r"\\",
        r":" => r"\:",
        r";" => r"\;",
        r"," => r"\,",
        _ => ch,
    })
}

/**
```
use rsmorphy::container::decode::unescape;

assert_eq!(unescape(r"a\,b").collect::<String>(),   String::from(r"a,b"));
assert_eq!(unescape(r"a\;b").collect::<String>(),   String::from(r"a;b"));
assert_eq!(unescape(r"a\:b").collect::<String>(),   String::from(r"a:b"));
assert_eq!(unescape(r"a\\b").collect::<String>(),   String::from(r"a\b"));
assert_eq!(unescape(r"a\")  .collect::<String>(),   String::from(r"a\"));
```
*/
pub fn unescape<'s: 'i, 'i>(s: &'s str) -> impl Iterator<Item = &'s str> + 'i {
    // trace!(r#"unescape: "{}""#, s);
    let i1 = s.split("");
    let i2 = i1.clone().skip(1);
    let mut esc = false;
    i1.zip(i2)
        .filter(move |&(c1, c2)| {
            // trace!(r#"c1, c2: "{}", "{}""#, c1, c2);
            // FIXME a bug in clippy; https://github.com/rust-lang-nursery/rust-clippy/issues/860
            #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
            match (esc, c1, c2) {
                (false, r"\", "") => true,
                (false, r"\", _) => {
                    esc = true;
                    false
                }
                (true, _, _) => {
                    esc = false;
                    true
                }
                _ => true,
            }
        })
        .map(|(c1, _)| c1)
}
