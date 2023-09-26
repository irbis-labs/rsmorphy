use smart_string::SmartString;

#[derive(Debug, Clone)]
pub struct HyphenSeparatedParticle {
    pub particle: SmartString,
}

impl HyphenSeparatedParticle {
    #[inline(always)]
    pub fn new(particle: impl Into<SmartString>) -> Self {
        let particle = particle.into();
        HyphenSeparatedParticle { particle }
    }
}
