use inlinable_string::InlinableString;

#[derive(Debug, Clone)]
pub struct HyphenSeparatedParticle {
    pub particle: InlinableString,
}

impl HyphenSeparatedParticle {
    #[inline(always)]
    pub fn new(particle: impl Into<InlinableString>) -> Self {
        let particle = particle.into();
        HyphenSeparatedParticle { particle }
    }
}
