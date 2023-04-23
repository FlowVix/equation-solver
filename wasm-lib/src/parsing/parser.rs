#![deny(unused_must_use)]

use std::ops::Range;

use logos::{Lexer, Logos};

use super::{
    ast::ExprNode,
    lexer::{NextOrEnd, Token},
};

#[derive(Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

pub type ParseResult<T> = Result<T, String>;

pub fn unexpected_err_str(found: Token, exp: &str) -> String {
    // let bt = Backtrace::new();

    // println!("{:?}", bt);

    format!("expected `{}`, found `{}`", exp, found.name())
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        let lexer = Token::lexer(code);
        Parser { lexer }
    }

    pub fn next(&mut self) -> Token {
        self.lexer.next_or_end()
    }

    pub fn span(&self) -> Range<usize> {
        self.lexer.span()
    }
    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn peek(&self) -> Token {
        let mut peek = self.lexer.clone();
        peek.next_or_end()
    }
    pub fn peek_slice(&self) -> String {
        let mut peek = self.lexer.clone();
        peek.next_or_end();
        peek.slice().into()
    }

    pub fn next_is(&self, tok: Token) -> bool {
        self.peek() == tok
    }
    pub fn next_are(&self, toks: &[Token]) -> bool {
        let mut peek = self.lexer.clone();
        for tok in toks {
            if peek.next_or_end() != *tok {
                return false;
            }
        }
        true
    }
    pub fn skip_tok(&mut self, skip: Token) -> bool {
        if self.next_is(skip) {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn expect_tok_named(&mut self, expect: Token, name: &str) -> ParseResult<()> {
        let next = self.next();
        if next != expect {
            return Err(unexpected_err_str(next, name));
        }
        Ok(())
    }

    pub fn expect_tok(&mut self, expect: Token) -> ParseResult<()> {
        self.expect_tok_named(expect, expect.name())
    }

    pub fn parse(&mut self) -> ParseResult<ExprNode> {
        todo!()
    }
}
