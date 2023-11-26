use crate::pest_parser::Rule;
use pest::error::Error;
use std::{collections::HashMap, fs, io::Result, path::PathBuf};

const DEFAULT_GRAMMAR_ID: &str = "this";

#[derive(Debug)]
pub(crate) struct ProcessedGrammar {
    pub variables: HashMap<String, GrammarVariable>,
    pub normal_rules: HashMap<String, GrammarRule>,
}

#[derive(Debug)]
pub(crate) enum PestGrammarError {
    Normal(Box<Error<()>>),
    Rule(Box<Error<Rule>>),
    Other(String),
}

#[derive(Debug, Clone)]
pub(crate) struct GrammarRule {
    pub rule: String,
    pub grammar: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GrammarVariable {
    pub value: String,
    pub grammar: String,
}

impl ToString for PestGrammarError {
    fn to_string(&self) -> String {
        match self {
            PestGrammarError::Normal(e) => e.to_string(),
            PestGrammarError::Rule(e) => e.to_string(),
            PestGrammarError::Other(e) => e.to_string(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Grammar {
    pub source: String,
    pub variables: HashMap<String, String>,
    pub(crate) id: String,
}

impl Grammar {
    pub fn from_source(source: &str) -> Self {
        Self {
            source: source.to_string(),
            id: String::from(DEFAULT_GRAMMAR_ID),
            ..Self::default()
        }
    }

    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        Ok(Self {
            source: fs::read_to_string(path.into())?,
            id: String::from(DEFAULT_GRAMMAR_ID),
            ..Self::default()
        })
    }
}
