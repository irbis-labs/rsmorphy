#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct HyphenSeparatedParticle {
    pub particle: String,
}

impl HyphenSeparatedParticle {
    pub fn new<P>(particle: P) -> Self
    where
        P: Into<String>,
    {
        let particle = particle.into();
        HyphenSeparatedParticle { particle }
    }

    pub fn title_rus(&self) -> &'static str {
        "частица"
    }
}
