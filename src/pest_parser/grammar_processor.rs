#![allow(clippy::map_entry)]

use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    parser::{
        parse_pest_source,
        types::{Grammar, GrammarRule, GrammarVariable, PestGrammarError, ProcessedGrammar},
    },
    pest_parser::{
        self,
        errors::{generate_already_exists_error, generate_pest_grammar_error},
    },
};

pub(crate) fn process_grammar(grammar: Grammar) -> Result<ProcessedGrammar, PestGrammarError> {
    let mut variables: HashMap<String, GrammarVariable> = HashMap::new();
    let mut normal_rules: HashMap<String, GrammarRule> = HashMap::new();
    match parse_pest_source(&grammar.source) {
        Ok(pairs) => {
            for e in pairs {
                let (span, rule, mut inner) = (
                    e.clone().as_span(),
                    e.clone().as_rule(),
                    e.clone().into_inner(),
                );
                match rule {
                    pest_parser::Rule::grammar_rule => {
                        let id = inner.next().unwrap().as_span().as_str().to_string();
                        let rule = span.as_str().to_string();

                        if normal_rules.contains_key(&id) {
                            let v = normal_rules.get(&id).unwrap();
                            return Err(generate_already_exists_error(
                                format!(
                                    "Rule \"{}\" from grammar \"{}\" already exists here:",
                                    id.clone(),
                                    v.clone().grammar
                                ),
                                span,
                                (
                                    format!("{} -> {}", v.clone().grammar, grammar.clone().id),
                                    v.clone().rule,
                                ),
                                (grammar.clone().id, format!("you want to set \"{}\"", rule)),
                            ));
                        } else if variables.contains_key(&id) {
                            let v = variables.get(&id).unwrap();
                            return Err(generate_already_exists_error(
                                format!(
                                    "Rule \"{}\" from grammar \"{}\" conflicts with variable \"{}\":",
                                    id.clone(),
                                    grammar.clone().id,
                                    id.clone(),
                                ),
                                span,
                                (
                                    v.clone().grammar,
                                    format!("\"{}\"", v.clone().value)
                                ),
                                (grammar.clone().id, format!("you want to set \"{}\"", rule)),
                            ));
                        } else {
                            normal_rules.insert(
                                id,
                                GrammarRule {
                                    rule,
                                    grammar: grammar.id.clone(),
                                },
                            );
                        }
                    }
                    pest_parser::Rule::variable_rule => {
                        let id = inner.next().unwrap().as_span().as_str().to_string();
                        let value = pest_parser::parse_string(inner.nth(1).unwrap());
                        if variables.contains_key(&id) {
                            let v = variables.get(&id).unwrap();
                            return Err(generate_already_exists_error(
                                format!(
                                    "Variable \"{}\" from grammar \"{}\" already exists here:",
                                    id.clone(),
                                    v.clone().grammar
                                ),
                                span,
                                (
                                    format!("{} -> {}", v.clone().grammar, grammar.clone().id),
                                    v.clone().value,
                                ),
                                (grammar.clone().id, format!("you want to set \"{}\"", value)),
                            ));
                        } else if normal_rules.contains_key(&id) {
                            let v = normal_rules.get(&id).unwrap();
                            return Err(generate_already_exists_error(
                                format!(
                                    "Variable \"{}\" from grammar \"{}\" conflicts with rule \"{}\":",
                                    id.clone(),
                                    grammar.clone().id,
                                    id.clone(),
                                ),
                                span,
                                (
                                    v.clone().grammar,
                                    format!("\"{}\"", v.clone().rule)
                                ),
                                (grammar.clone().id, format!("you want to set \"{}\"", value)),
                            ));
                        } else {
                            variables.insert(
                                id,
                                GrammarVariable {
                                    value,
                                    grammar: grammar.id.clone(),
                                },
                            );
                        }
                    }
                    pest_parser::Rule::include => {
                        let path_str = pest_parser::parse_string(inner.next().unwrap());
                        let path = PathBuf::from(path_str.clone());
                        if path.exists() {
                            let included_grammar = Grammar {
                                source: fs::read_to_string(path).unwrap(),
                                id: path_str.clone(),
                                ..Grammar::default()
                            };
                            match process_grammar(included_grammar) {
                                Ok(processed) => {
                                    for (k, v) in processed.normal_rules.iter() {
                                        if normal_rules.contains_key(k) {
                                            return Err(generate_already_exists_error(
                                                format!(
                                                    "Rule \"{}\" from grammar \"{}\" already exists here:",
                                                    k,
                                                    path_str.clone()
                                                ),
                                                span,
                                                (
                                                    grammar.clone().id,
                                                    normal_rules.get(k).unwrap().rule.clone(),
                                                ),
                                                (path_str.clone(), v.rule.clone()),
                                            ));
                                        } else {
                                            normal_rules.insert(k.to_string(), v.clone());
                                        }
                                    }
                                    for (k, v) in processed.variables.iter() {
                                        if variables.contains_key(k) {
                                            return Err(generate_already_exists_error(format!(
                                                "Variable \"{}\" from grammar \"{}\" already exists here:",
                                                k,
                                                path_str.clone()
                                            ), span, (grammar.clone().id, variables.get(k).unwrap().value.clone()), (path_str.clone(), v.value.clone())));
                                        } else {
                                            variables.insert(k.to_string(), v.clone());
                                        }
                                    }
                                }
                                Err(err) => {
                                    return Err(PestGrammarError::Other(format!(
                                    "An error occurred while parsing included grammar \"{}\":\n{}",
                                    path_str,
                                    err.to_string()
                                )))
                                }
                            }
                        } else {
                            return Err(generate_pest_grammar_error(
                                span,
                                format!("Path doesn't exists: {}", path_str),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(err) => return Err(PestGrammarError::Rule(err)),
    }
    Ok(ProcessedGrammar {
        variables,
        normal_rules,
    })
}
