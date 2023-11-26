pub mod types;

use crate::pest_parser::{self, grammar_processor::process_grammar};

use pest::Parser;

use types::*;

fn build_grammar(processed: ProcessedGrammar) -> String {
    let mut grammar = String::new();
    for (k, v) in processed.variables.iter() {
        grammar += format!("{k} = {{ \"{}\" }}\n", v.value).as_str();
    }
    for (_, v) in processed.normal_rules.iter() {
        grammar += format!("{}\n", v.rule).as_str();
    }
    grammar
}

pub fn compile_grammar(grammar: Grammar) -> Result<String, String> {
    match process_grammar(grammar) {
        Ok(processed) => Ok(build_grammar(processed)),
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_pest_source(
    source: &str,
) -> Result<pest::iterators::Pairs<'_, pest_parser::Rule>, Box<pest::error::Error<pest_parser::Rule>>>
{
    let res = pest_parser::PestParser::parse(pest_parser::Rule::grammar_rules, source);
    if let Err(e) = res {
        Err(Box::new(e))
    } else {
        Ok(res.unwrap())
    }
}
