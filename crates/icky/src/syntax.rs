mod lexer;

use crate::error::{Error, Result};
use lexer::{lex, Span, Token};

/// A name that starts with an "uppercase" letter.
///
/// Typically, this is used for type names in the language.
#[derive(Debug)]
pub struct UpperIdent(pub Span);

/// A name that starts with a "lowercase" letter.
///
/// Typically, this is used for lower names in the language.
#[derive(Debug)]
pub struct LowerIdent(pub Span);

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Declaration {
    pub header_name: LowerIdent,
    pub header_type: UpperIdent,
    pub body_name: LowerIdent,
    pub body: Expr,
}

#[derive(Debug)]
pub struct SyntaxTree {
    pub declarations: Vec<Declaration>,
}

peg::parser! {
    grammar root() for [Token] {
        use Token::*;

        rule integer_lit() -> Expr
            = [IntegerLiteral(i)] { Expr::IntegerLiteral(i) }

        rule lower_ident() -> LowerIdent
            = [LowerName(n)] { LowerIdent(n) }

        rule upper_ident() -> UpperIdent
            = [UpperName(n)] { UpperIdent(n) }

        rule expr() -> Expr
            = e:integer_lit() { e }

        rule sep() -> ()
            = [Semicolon | LineBreak] { () }

        rule declaration() -> Declaration
            = header_name:lower_ident()
              [Colon]
              header_type:upper_ident()
              sep()
              body_name:lower_ident()
              [Equals]
              body:expr()
              {
                  Declaration { header_name, header_type, body_name, body }
              }

        pub rule root() -> SyntaxTree
            = declarations:declaration()* { SyntaxTree { declarations } }
    }
}

pub fn parse(source: &str) -> Result<SyntaxTree> {
    let tokens = lex(source).collect::<Result<Vec<_>>>()?;
    root::root(&tokens).map_err(|e| Error(format!("{}", e)))
}
