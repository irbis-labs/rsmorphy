pub mod error;
pub mod follow;
pub mod take;
pub mod map;
pub mod predicate;


pub use self::follow::*;
pub use self::take::*;
pub use self::map::*;
pub use self::predicate::*;



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
    s.split("").map(|ch| {
        match ch {
            r"\" => r"\\",
            r":" => r"\:",
            r";" => r"\;",
            r"," => r"\,",
            _ => ch
        }
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
    let i2 = s.split("").skip(1);
    let mut esc = false;
    i1.zip(i2)
        .filter(move |&(c1, c2)| {
            // trace!(r#"c1, c2: "{}", "{}""#, c1, c2);
            match (esc, c1, c2) {
                (false, r"\", "")   => {                true },
                (false, r"\", _ )   => { esc = true;    false },
                (true,  _,    _ )   => { esc = false;   true },
                _                   => {                true },
            }
        })
        .map(|(c1, _)| c1)
}
