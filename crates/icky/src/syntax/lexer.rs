//! This module exists because writing a parser without a lexer is very annoying:
//!   1. you need to manually implement whitespace skipping,
//!   2. in our case, we need semi-colon insertion, which we do by inserting "new line" tokens.
use crate::error::Result;
use std::{
    iter::{self, Peekable},
    str::CharIndices,
};

fn starts_lower_name(c: char) -> bool {
    c.is_lowercase()
}

fn starts_upper_name(c: char) -> bool {
    c.is_uppercase()
}

fn continues_name(c: char) -> bool {
    c.is_alphanumeric()
}

fn digit(c: char) -> Option<i64> {
    Some(c.to_digit(10)? as i64)
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    UpperName(Span),
    LowerName(Span),
    IntegerLiteral(i64),
    LineBreak,
    Colon,
    Semicolon,
}

struct Lexer<'s> {
    inner: Peekable<CharIndices<'s>>,
}

impl<'s> Lexer<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            inner: source.char_indices().peekable(),
        }
    }

    fn name(&mut self, start: usize, first: char) -> Span {
        let (end, _) = iter::once((start, first))
            .chain(self.inner.by_ref().take_while(|&(_, c)| continues_name(c)))
            .last()
            .unwrap();
        Span {
            start,
            len: end - start + 1,
        }
    }

    /// Collect whitespace, returning true if a line break was encountered.
    fn whitespace(&mut self, start: char) -> bool {
        iter::once(start)
            .chain(
                self.inner
                    .by_ref()
                    .map(|(_, c)| c)
                    .take_while(|c| c.is_whitespace()),
            )
            .any(|c| c == '\n')
    }

    fn integer(&mut self, start: char) -> i64 {
        iter::once(start)
            .chain(self.inner.by_ref().map(|(_, c)| c))
            .map_while(|c| digit(c))
            .fold(0i64, |acc, x| 10 * acc + x)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // We exit only by returning.
        loop {
            let (start, c) = self.inner.next()?;
            match c {
                ':' => return Some(Ok(Token::Colon)),
                ';' => return Some(Ok(Token::Semicolon)),
                c if starts_lower_name(c) => {
                    return Some(Ok(Token::LowerName(self.name(start, c))))
                }
                c if starts_upper_name(c) => {
                    return Some(Ok(Token::UpperName(self.name(start, c))))
                }
                c if c.is_whitespace() => {
                    if self.whitespace(c) {
                        return Some(Ok(Token::LineBreak));
                    }
                }
                c if c.is_digit(10) => return Some(Ok(Token::IntegerLiteral(self.integer(c)))),
                c => return Some(Err(format!("lexer: unexpected character: {}", c).into())),
            };
        }
    }
}

pub fn lex<'s>(source: &'s str) -> impl Iterator<Item = Result<Token>> + 's {
    Lexer::new(source)
}
