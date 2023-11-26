use crate::pest_parser::{PestParser, Rule};
use pest::{iterators::Pair, Parser};
use pest_fmt::{Formatter, PestError};

/// Format grammar
pub fn format(src: String) -> Result<String, PestError> {
    Formatter::new(&src).format()
}

fn minify_pair(pair: Pair<'_, Rule>) -> String {
    let mut minified = String::new();
    let rule = pair.clone().as_rule();
    let inner = pair.clone().into_inner();
    if rule != Rule::line_doc && rule != Rule::grammar_doc {
        if inner.len() == 0 {
            minified.push_str(pair.as_span().as_str());
        } else {
            for inner_pair in inner {
                minified.push_str(&minify_pair(inner_pair));
            }
        }
    }
    minified
}

/// Minify grammar
pub fn minify(src: String) -> Result<String, pest::error::Error<Rule>> {
    let mut minified = String::new();
    match PestParser::parse(Rule::grammar_rules, &src) {
        Ok(pairs) => {
            for pair in pairs {
                minified.push_str(&minify_pair(pair));
            }
            Ok(minified)
        }
        Err(e) => Err(e),
    }
}
