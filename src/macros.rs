#[macro_export]
macro_rules! regex {
    ($name:ident, $re:expr) => {
        lazy_static::lazy_static! {
            static ref $name: ::regex::Regex = ::regex::Regex::new($re).unwrap();
        }
    };
}
