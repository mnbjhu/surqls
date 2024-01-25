use std::fmt::Display;

use super::keyword::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Keyword(Keyword),
    Operator(String),
    Punctuation(char),
    Newline,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{}", i),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Boolean(b) => write!(f, "{}", b),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Keyword(k) => write!(f, "{}", k),
            Token::Operator(s) => write!(f, "{}", s),
            Token::Punctuation(c) => write!(f, "{}", c),
            Token::Newline => write!(f, "\\n"),
        }
    }
}
