use chumsky::{select, Parser};

use crate::core::{lexer::Token, parser::parser::Extra, span::ParserInput};

pub fn literal_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Literal, Extra<'tokens>> + Clone {
    select! {
        Token::Integer(i) => Literal::Int(i),
        Token::Float(f) => Literal::Float(f),
        Token::String(s) => Literal::String(s.to_string()),
        Token::Boolean(b) => Literal::Bool(b),
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}
