use pest::Span;

use crate::parser::types::PestGrammarError;

pub(crate) fn generate_pest_grammar_error(span: Span, message: String) -> PestGrammarError {
    PestGrammarError::Normal(Box::new(pest::error::Error::new_from_span(
        pest::error::ErrorVariant::<()>::CustomError { message },
        span,
    )))
}

pub(crate) fn generate_already_exists_error(
    msg: String,
    span: Span,
    (cmp1_index, cmp1_value): (String, String),
    (cmp2_index, cmp2_value): (String, String),
) -> PestGrammarError {
    PestGrammarError::Normal(Box::new(pest::error::Error::new_from_span(
        pest::error::ErrorVariant::<()>::CustomError {
            message: format!(
                "{msg}\n[{}]: {}\n[{}]: {}",
                cmp1_index, cmp1_value, cmp2_index, cmp2_value
            ),
        },
        span,
    )))
}
