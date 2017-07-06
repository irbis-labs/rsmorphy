/**
    Ожидаемые варианты стека без учёта сдвоенных слов:
    * Dictinary[, KnownPrefix|UnknownPrefix][, HyphenSeparatedParticle]
    * FakeDictinary[, KnownSuffix][, HyphenSeparatedParticle]
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
