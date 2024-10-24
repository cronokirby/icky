//! This module exists because writing a parser without a lexer is very annoying:
//!   1. you need to manually implement whitespace skipping,
//!   2. in our case, we need semi-colon insertion, which we do by inserting "new line" tokens.
use crate::error::Result;
use std::{
    iter::{self, Peekable},
    str::Chars,
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

#[derive(Debug)]
pub enum Token {
    UpperName(String),
    LowerName(String),
    IntegerLiteral(i64),
    LineBreak,
    Colon,
    Semicolon,
}

struct Lexer<'s> {
    inner: Peekable<Chars<'s>>,
}

impl<'s> Lexer<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            inner: source.chars().peekable(),
        }
    }

    fn name(&mut self, start: char) -> String {
        iter::once(start)
            .chain(self.inner.by_ref().take_while(|&c| continues_name(c)))
            .collect()
    }

    /// Collect whitespace, returning true if a line break was encountered.
    fn whitespace(&mut self, start: char) -> bool {
        iter::once(start)
            .chain(self.inner.by_ref().take_while(|&c| c.is_whitespace()))
            .any(|c| c == '\n')
    }

    fn integer(&mut self, start: char) -> i64 {
        iter::once(start)
            .chain(self.inner.by_ref())
            .map_while(|c| digit(c))
            .fold(0i64, |acc, x| 10 * acc + x)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // We exit only by returning.
        loop {
            match self.inner.next()? {
                ':' => return Some(Ok(Token::Colon)),
                ';' => return Some(Ok(Token::Semicolon)),
                c if starts_lower_name(c) => return Some(Ok(Token::LowerName(self.name(c)))),
                c if starts_upper_name(c) => return Some(Ok(Token::UpperName(self.name(c)))),
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
