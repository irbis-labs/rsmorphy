use crate::{
    analyzer::MorphAnalyzer,
    container::{ParseResult, SeenSet},
};

pub trait AnalyzerUnit {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        seen_parses: &mut SeenSet,
    );
}
