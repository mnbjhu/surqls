use chumsky::{select, Parser};

use crate::{
    ast::{expr::literal::Literal, parser::Extra},
    lexer::token::Token,
    util::span::ParserInput,
};

pub fn literal_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Literal, Extra<'tokens>> + Clone {
    select! {
        Token::Integer(i) => Literal::Int(i),
        Token::Float(f) => Literal::Float(f),
        Token::String(s) => Literal::String(s.to_string()),
        Token::Boolean(b) => Literal::Bool(b),
    }
}
