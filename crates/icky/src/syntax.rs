mod lexer;

use crate::error::Result;

/// A name that starts with an "uppercase" letter.
///
/// Typically, this is used for names in the language.
pub struct UpperName(pub String);

pub struct LowerName(pub String);

pub enum Expr {
    // TODO[1]: allow arbitrary precision here.
    /// e.g. `0`, `1000`, etc.
    IntegerLiteral(i64),
}

/// A top level declaration, e.g.
/// ```
/// a : Int
/// a = 0
/// ```
pub struct Declaration {
    pub header_name: LowerName,
    pub header_type: UpperName,
    pub body_name: LowerName,
    pub body: Expr,
}

pub struct SyntaxTree {
    pub declarations: Vec<Declaration>,
}

pub fn parse(_source: &str) -> Result<SyntaxTree> {
    todo!()
}
