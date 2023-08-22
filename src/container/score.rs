use std::cmp::Ordering;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Score {
    /// Evaluated from the dictionary or other source.
    Real(f64),
    /// Evaluated out of thin air.
    Fake(f64),
}

impl Score {
    pub fn value(&self) -> f64 {
        match *self {
            Score::Real(v) | Score::Fake(v) => v,
        }
    }

    pub fn value_ref_mut(&mut self) -> &mut f64 {
        match *self {
            Score::Real(ref mut v) | Score::Fake(ref mut v) => v,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Mul for Score {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Score::Real(a), Score::Real(b)) => Score::Real(a * b),
            _ => Score::Fake(self.value() * rhs.value()),
        }
    }
}

impl Mul<f64> for Score {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Score::Real(v) => Score::Real(v * rhs),
            Score::Fake(v) => Score::Fake(v * rhs),
        }
    }
}
