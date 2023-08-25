use inlinable_string::InlinableString;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct HyphenSeparatedParticle {
    pub(in crate::analyzer) particle: InlinableString,
}

impl HyphenSeparatedParticle {
    #[inline(always)]
    pub fn new<P>(particle: P) -> Self
    where
        P: Into<InlinableString>,
    {
        let particle = particle.into();
        HyphenSeparatedParticle { particle }
    }

    #[inline(always)]
    pub fn particle(&self) -> &str {
        &self.particle
    }

    #[inline(always)]
    pub fn into_inner(self) -> InlinableString {
        self.particle
    }

    #[inline(always)]
    pub fn title_rus(&self) -> &'static str {
        "частица"
    }
}
