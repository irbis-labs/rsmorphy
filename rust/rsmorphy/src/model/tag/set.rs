use crate::model::gram::GramId;
use crate::model::kind::Animacy;
use crate::model::kind::Aspect;
use crate::model::kind::Case;
use crate::model::kind::Gender;
use crate::model::kind::Involvement;
use crate::model::kind::Mood;
use crate::model::kind::Number;
use crate::model::kind::PartOfSpeech;
use crate::model::kind::Person;
use crate::model::kind::Tense;
use crate::model::kind::Transitivity;
use crate::model::kind::Voice;
use crate::model::tag::EitherError;

#[derive(Debug, Default, Clone, Copy)]
pub struct TagSet {
    pub pos: Option<PartOfSpeech>,
    pub animacy: Option<Animacy>,
    pub aspect: Option<Aspect>,
    pub case: Option<Case>,
    pub gender: Option<Gender>,
    pub involvement: Option<Involvement>,
    pub mood: Option<Mood>,
    pub number: Option<Number>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub transitivity: Option<Transitivity>,
    pub voice: Option<Voice>,

    pub has_apro: bool,
}

impl TagSet {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_gram_id(gram_id: GramId) -> Self {
        Self {
            pos: PartOfSpeech::from_gram_id(gram_id),
            animacy: Animacy::from_gram_id(gram_id),
            aspect: Aspect::from_gram_id(gram_id),
            case: Case::from_gram_id(gram_id),
            gender: Gender::from_gram_id(gram_id),
            involvement: Involvement::from_gram_id(gram_id),
            mood: Mood::from_gram_id(gram_id),
            number: Number::from_gram_id(gram_id),
            person: Person::from_gram_id(gram_id),
            tense: Tense::from_gram_id(gram_id),
            transitivity: Transitivity::from_gram_id(gram_id),
            voice: Voice::from_gram_id(gram_id),
            has_apro: gram_id.as_str() == "Apro",
        }
    }

    pub fn join(self, other: Self) -> Result<Self, EitherError> {
        Ok(Self {
            pos: either(self.pos, other.pos)?,
            animacy: either(self.animacy, other.animacy)?,
            aspect: either(self.aspect, other.aspect)?,
            case: either(self.case, other.case)?,
            gender: either(self.gender, other.gender)?,
            involvement: either(self.involvement, other.involvement)?,
            mood: either(self.mood, other.mood)?,
            number: either(self.number, other.number)?,
            person: either(self.person, other.person)?,
            tense: either(self.tense, other.tense)?,
            transitivity: either(self.transitivity, other.transitivity)?,
            voice: either(self.voice, other.voice)?,
            has_apro: self.has_apro || other.has_apro,
        })
    }

    #[inline(always)]
    pub fn is_productive(&self) -> bool {
        self.pos
            .map(PartOfSpeech::is_productive)
            .unwrap_or_else(|| !self.has_apro)
    }
}

#[inline(always)]
fn either<T>(a: Option<T>, b: Option<T>) -> Result<Option<T>, EitherError>
where
    T: PartialEq,
{
    Ok(match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => {
            if a == b {
                Some(a)
            } else {
                return Err(EitherError);
            }
        }
    })
}
