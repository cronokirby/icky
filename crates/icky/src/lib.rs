mod context;
mod pretty;
mod syntax;

use anyhow;
pub use context::Context;
pub use pretty::Doc;
use std::collections::HashMap;
pub use syntax::{parse, SyntaxTree};

/// A value in our programming language.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(i64),
}

/// A context maps names to their evaluations.
#[derive(Debug, Clone)]
pub struct EvalContext {
    names: HashMap<String, Value>,
}

impl EvalContext {
    /// Get the value of a name in this context, if any.
    pub fn get(&self, name: &str) -> Option<Value> {
        self.names.get(name).copied()
    }
}

fn eval_tree(_tree: SyntaxTree) -> anyhow::Result<EvalContext> {
    todo!()
}

/// Take in a source file, and produce the evaluation context.
///
/// This context will map each definition in the file to its evaluation.
pub fn eval(source: &str) -> anyhow::Result<EvalContext> {
    let tree = parse(source)?;
    eval_tree(tree)
}
