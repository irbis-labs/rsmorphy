use std::ops::Mul;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Score {
    Real(f64),
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
