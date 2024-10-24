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
    Equals,
}

impl Token {
    pub fn integer_literal(self) -> Option<i64> {
        match self {
            Token::IntegerLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub fn name(self) -> Option<Span> {
        match self {
            Token::UpperName(x) => Some(x),
            Token::LowerName(x) => Some(x),
            _ => None,
        }
    }
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

    fn name(&mut self, start: usize, _first: char) -> Span {
        let mut len = 1;
        while let Some(_) = self.inner.next_if(|(_, c)| continues_name(*c)) {
            len += 1;
        }
        Span { start, len }
    }

    /// Collect whitespace, returning true if a line break was encountered.
    fn whitespace(&mut self, start: char) -> bool {
        let mut has_newline = start == '\n';
        while let Some((_, c)) = self.inner.next_if(|(_, c)| c.is_whitespace()) {
            has_newline = has_newline || c == '\n';
        }
        has_newline
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
                '=' => return Some(Ok(Token::Equals)),
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
