use derive_more::From;

mod animacy;
mod aspect;
mod case;
mod gender;
mod initial;
mod involvement;
mod mood;
mod number;
mod person;
mod pos;
mod tense;
mod transitivity;
mod voice;

pub use self::animacy::Animacy;
pub use self::aspect::Aspect;
pub use self::case::Case;
pub use self::gender::Gender;
pub use self::initial::InitialKind;
pub use self::involvement::Involvement;
pub use self::mood::Mood;
pub use self::number::Number;
pub use self::person::Person;
pub use self::pos::PartOfSpeech;
pub use self::tense::Tense;
pub use self::transitivity::Transitivity;
pub use self::voice::Voice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, From)]
pub enum TagKind {
    Animacy(Animacy),
    Aspect(Aspect),
    Case(Case),
    Gender(Gender),
    Initial(InitialKind),
    Involvement(Involvement),
    Mood(Mood),
    Number(Number),
    Person(Person),
    PartOfSpeech(PartOfSpeech),
    Tense(Tense),
    Transitivity(Transitivity),
    Voice(Voice),
}

impl TagKind {
    pub fn gram_int(self) -> &'static str {
        match self {
            TagKind::Animacy(kind) => kind.gram_int(),
            TagKind::Aspect(kind) => kind.gram_int(),
            TagKind::Case(kind) => kind.gram_int(),
            TagKind::Gender(kind) => kind.gram_int(),
            TagKind::Initial(kind) => kind.gram_int(),
            TagKind::Involvement(kind) => kind.gram_int(),
            TagKind::Mood(kind) => kind.gram_int(),
            TagKind::Number(kind) => kind.gram_int(),
            TagKind::Person(kind) => kind.gram_int(),
            TagKind::PartOfSpeech(kind) => kind.gram_int(),
            TagKind::Tense(kind) => kind.gram_int(),
            TagKind::Transitivity(kind) => kind.gram_int(),
            TagKind::Voice(kind) => kind.gram_int(),
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            TagKind::Animacy(kind) => kind.title_rus(),
            TagKind::Aspect(kind) => kind.title_rus(),
            TagKind::Case(kind) => kind.title_rus(),
            TagKind::Gender(kind) => kind.title_rus(),
            TagKind::Initial(kind) => kind.title_rus(),
            TagKind::Involvement(kind) => kind.title_rus(),
            TagKind::Mood(kind) => kind.title_rus(),
            TagKind::Number(kind) => kind.title_rus(),
            TagKind::Person(kind) => kind.title_rus(),
            TagKind::PartOfSpeech(kind) => kind.title_rus(),
            TagKind::Tense(kind) => kind.title_rus(),
            TagKind::Transitivity(kind) => kind.title_rus(),
            TagKind::Voice(kind) => kind.title_rus(),
        }
    }
}
