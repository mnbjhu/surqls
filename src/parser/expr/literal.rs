use chumsky::{select, Parser};

use crate::{
    ast::{expr::literal::Literal, parser::Extra},
    lexer::{keyword::Keyword, token::Token},
    util::span::ParserInput,
};

pub fn literal_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Literal, Extra<'tokens>> + Clone {
    select! {
        Token::Integer(i) => Literal::Int(i),
        Token::Float(f) => Literal::Float(f),
        Token::Decimal(d) => Literal::Decimal(d),
        Token::String(s) => Literal::String(s.to_string()),
        Token::Boolean(b) => Literal::Bool(b),
        Token::DateTime(dt) => Literal::DateTime(dt.to_string()),
        Token::Duration(d) => Literal::Duration(d.to_string()),
        Token::RecordString(s) => Literal::RecordString(s.to_string()),
        Token::Keyword(Keyword::Null) => Literal::Null,
    }
}
