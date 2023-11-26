pub mod errors;
pub mod grammar_processor;

use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest_parser/pest.pest"]
pub struct PestParser;

/// Parse string from modified pest meta grammar
pub fn parse_string(pair: Pair<'_, Rule>) -> String {
    pair.into_inner()
        .nth(1)
        .unwrap()
        .as_span()
        .as_str()
        .to_string()
}
