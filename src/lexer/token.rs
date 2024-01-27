use std::fmt::Display;

use super::keyword::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    DateTime(String),
    Boolean(bool),
    Identifier(String),
    Variable(String),
    Keyword(Keyword),
    Operator(String),
    Punctuation(char),
    Duration(String),
    RecordString(String),
    Newline,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::DateTime(i) => write!(f, "{}", i),
            Token::RecordString(i) => write!(f, "{}", i),
            Token::Duration(i) => write!(f, "{}", i),
            Token::Integer(i) => write!(f, "{}", i),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Boolean(b) => write!(f, "{}", b),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Keyword(k) => write!(f, "{}", k),
            Token::Operator(s) => write!(f, "{}", s),
            Token::Punctuation(c) => write!(f, "{}", c),
            Token::Variable(s) => write!(f, "${}", s),
            Token::Newline => write!(f, "\\n"),
        }
    }
}
