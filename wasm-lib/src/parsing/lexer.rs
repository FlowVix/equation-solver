use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r#"\d+(\.\d+)?|\.\d+"#)]
    Number,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,
    #[token("^")]
    Pow,
    #[token("%")]
    Mod,

    #[token("e")]
    E,
    #[token("pi")]
    Pi,
    #[token("i")]
    I,

    #[token("(")]
    OpenParen,
    #[token(")")]
    ClosedParen,

    #[token("|")]
    Pipe,

    // Or regular expressions.
    #[regex(r#"[A-Za-z_][A-Za-z0-9_']*"#)]
    Identifier,

    End,
    Error,
}

pub trait NextOrEnd {
    fn next_or_end(&mut self) -> Token;
}

impl NextOrEnd for Lexer<'_, Token> {
    fn next_or_end(&mut self) -> Token {
        self.next()
            .map(|r| r.unwrap_or(Token::Error))
            .unwrap_or(Token::End)
    }
}

impl Token {
    pub fn name(&self) -> &'static str {
        match self {
            Token::Number => "number",
            Token::Plus => "`+`",
            Token::Minus => "`-`",
            Token::Mult => "`*`",
            Token::Div => "`/`",
            Token::Pow => "`^`",
            Token::Mod => "`%`",
            Token::E => "e",
            Token::Pi => "pi",
            Token::I => "i",
            Token::Pipe => "`|`",
            Token::OpenParen => "`(`",
            Token::ClosedParen => "`)`",
            Token::Identifier => "variable",
            Token::End => "equation end",
            Token::Error => "unknown",
        }
    }
}
