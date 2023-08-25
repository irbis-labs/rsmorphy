/*
    Ожидаемые варианты стека без учёта сдвоенных слов:
    * Dictionary[, KnownPrefix|UnknownPrefix][, HyphenSeparatedParticle]
    * FakeDictionary[, KnownSuffix][, HyphenSeparatedParticle]
    * Shaped[, HyphenSeparatedParticle]
    * Unknown[, HyphenSeparatedParticle]
    * Initials
*/
pub mod affix;
pub mod hsp;
pub mod hword;
pub mod source;

pub use self::affix::StackAffix;
pub use self::hsp::StackParticle;
pub use self::hword::StackHyphenated;
pub use self::source::StackSource;
