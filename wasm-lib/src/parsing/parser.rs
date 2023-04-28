#![deny(unused_must_use)]

use std::{collections::HashMap, ops::Range};

use logos::{Lexer, Logos};

use crate::parsing::ast::Function;

use super::{
    ast::ExprNode,
    lexer::{NextOrEnd, Token},
    operators,
};

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    name_map: &'a mut HashMap<String, u16>,
}

pub type ParseResult<T> = Result<T, String>;

pub fn unexpected_err_str(found: Token, exp: &str) -> String {
    format!("Expected {}, found {}", exp, found.name())
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str, map: &'a mut HashMap<String, u16>) -> Self {
        let lexer = Token::lexer(code);
        Parser {
            lexer,
            name_map: map,
        }
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

    pub fn get_name_id(&mut self, name: String) -> u16 {
        match self.name_map.get(&name) {
            Some(v) => *v,
            None => {
                let id = self.name_map.len() as u16;
                self.name_map.insert(name, id);
                id
            }
        }
    }

    pub fn parse_unit(&mut self) -> ParseResult<ExprNode> {
        Ok(match self.next() {
            Token::Number => {
                let mut expr = ExprNode::Number(self.slice().parse().unwrap());
                match self.peek() {
                    Token::I => {
                        self.next();
                        expr = ExprNode::BinOp(
                            Box::new(expr),
                            operators::BinOp::Mult,
                            Box::new(ExprNode::I),
                        )
                    }
                    Token::Identifier => {
                        self.next();
                        expr = ExprNode::BinOp(
                            Box::new(expr),
                            operators::BinOp::Mult,
                            Box::new(ExprNode::Var(self.get_name_id(self.slice().into()))),
                        )
                    }
                    _ => (),
                }
                expr
            }
            Token::E => ExprNode::E,
            Token::Pi => ExprNode::Pi,
            Token::I => ExprNode::I,
            Token::OpenParen => {
                let v = self.parse_expr()?;
                self.expect_tok(Token::ClosedParen)?;
                v
            }
            Token::Pipe => {
                let v = self.parse_expr()?;
                self.expect_tok(Token::Pipe)?;
                ExprNode::Abs(Box::new(v))
            }
            Token::Identifier => {
                let v = self.slice().to_string();
                if self.skip_tok(Token::OpenParen) {
                    let Some(func) = Function::from_str(&v) else {
                        return Err(format!("Unknown function `{}`", v))
                    };
                    let v = self.parse_expr()?;
                    self.expect_tok(Token::ClosedParen)?;
                    ExprNode::Func(func, Box::new(v))
                } else {
                    if Function::from_str(&v).is_some() {
                        return Err(format!("Cannot use variable with function name `{}`", v));
                    }
                    ExprNode::Var(self.get_name_id(v))
                }
            }
            t => return Err(unexpected_err_str(t, "expression")),
        })
    }

    pub fn parse_expr(&mut self) -> ParseResult<ExprNode> {
        self.parse_op(0)
    }

    pub fn parse_op(&mut self, prec: usize) -> ParseResult<ExprNode> {
        let next_prec = operators::next_infix(prec);

        let mut left = match next_prec {
            Some(next_prec) => self.parse_op(next_prec)?,
            None => self.parse_unit()?,
        };

        while operators::is_infix_prec(self.peek(), prec) {
            let op = self.next();
            let right = if operators::prec_type(prec) == operators::OpType::Left {
                match next_prec {
                    Some(next_prec) => self.parse_op(next_prec)?,
                    None => self.parse_unit()?,
                }
            } else {
                self.parse_op(prec)?
            };
            left = ExprNode::BinOp(Box::new(left), op.to_bin_op().unwrap(), Box::new(right))
        }

        Ok(left)
    }

    pub fn parse(&mut self) -> ParseResult<ExprNode> {
        let out = self.parse_expr()?;
        self.expect_tok(Token::End)?;
        Ok(out)
    }
}
