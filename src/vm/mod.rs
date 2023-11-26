use pest_meta::{
    optimizer,
    parser::{self, Rule},
};
use pest_vm::Vm;

/// Pest Vm wrapper
pub struct PestVm {
    pub vm: Vm,
}

impl PestVm {
    /// Create new pest Vm
    pub fn new(grammar: &str) -> Self {
        let pairs = parser::parse(Rule::grammar_rules, grammar).unwrap();
        let ast = parser::consume_rules(pairs).unwrap();

        Self {
            vm: Vm::new(optimizer::optimize(ast)),
        }
    }
}
