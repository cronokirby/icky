mod error;
// TODO: remove when this is used.
#[allow(dead_code)]
mod syntax;

use error::Result;
use std::collections::HashMap;
use syntax::{parse, SyntaxTree};

/// A value in our programming language.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(i64),
}

/// A context maps names to their evaluations.
#[derive(Debug, Clone)]
pub struct Context {
    names: HashMap<String, Value>,
}

impl Context {
    /// Get the value of a name in this context, if any.
    pub fn get(&self, name: &str) -> Option<Value> {
        self.names.get(name).copied()
    }
}

fn eval_tree(_tree: SyntaxTree) -> Result<Context> {
    todo!()
}

/// Take in a source file, and produce the evaluation context.
///
/// This context will map each definition in the file to its evaluation.
pub fn eval(source: &str) -> Result<Context> {
    let tree = parse(source)?;
    eval_tree(tree)
}
