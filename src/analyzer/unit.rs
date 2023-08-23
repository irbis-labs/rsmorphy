use crate::analyzer::context::ParseContext;

pub trait AnalyzerUnit {
    fn parse(&self, context: &mut ParseContext<'_, '_>);
}
