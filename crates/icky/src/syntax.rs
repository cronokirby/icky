mod lexer;

use anyhow::anyhow;
use lexer::{lex, Span, Token};

use crate::{context::Context, pretty::Doc};

/// A name that starts with an "uppercase" letter.
///
/// Typically, this is used for type names in the language.
#[derive(Debug)]
pub struct UpperIdent(pub Span);

impl UpperIdent {
    fn pretty(&self, ctx: &Context) -> Doc {
        Doc::string(ctx.span(self.0.start, self.0.len).into())
    }
}

/// A name that starts with a "lowercase" letter.
///
/// Typically, this is used for lower names in the language.
#[derive(Debug)]
pub struct LowerIdent(pub Span);

impl LowerIdent {
    fn pretty(&self, ctx: &Context) -> Doc {
        Doc::string(ctx.span(self.0.start, self.0.len).into())
    }
}

#[derive(Debug)]
pub enum Expr {
    // TODO[1]: allow arbitrary precision here.
    /// e.g. `0`, `1000`, etc.
    IntegerLiteral(i64),
}

impl Expr {
    fn pretty(&self, _ctx: &Context) -> Doc {
        match self {
            Expr::IntegerLiteral(x) => Doc::string(format!("(i64 {})", x)),
        }
    }
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

impl Declaration {
    pub fn pretty(&self, ctx: &Context) -> Doc {
        Doc::join(
            "\n",
            vec![
                Doc::from("((: ")
                    .then(self.header_name.pretty(ctx))
                    .then(" ".into())
                    .then(self.header_type.pretty(ctx))
                    .then(")".into()),
                Doc::indent(
                    Doc::from("(= ")
                        .then(self.body_name.pretty(ctx))
                        .then(" ".into())
                        .then(self.body.pretty(ctx))
                        .then(")".into()),
                ),
            ],
        )
    }
}

#[derive(Debug)]
pub struct SyntaxTree {
    pub declarations: Vec<Declaration>,
}

impl SyntaxTree {
    pub fn pretty(&self, ctx: &Context) -> Doc {
        Doc::from("(decls\n")
            .then(Doc::indent(Doc::join(
                "\n",
                self.declarations.iter().map(|x| x.pretty(ctx)).collect(),
            )))
            .then(Doc::constant(")"))
    }
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

pub fn parse(source: &str) -> anyhow::Result<SyntaxTree> {
    let tokens = lex(source).collect::<Result<Vec<_>, _>>()?;
    root::root(&tokens).map_err(|e| anyhow!("{}", e))
}
