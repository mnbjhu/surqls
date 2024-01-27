use chumsky::{
    primitive::{any, choice, one_of},
    recovery::skip_then_retry_until,
    select, Parser,
};

use crate::{
    ast::{
        parser::Extra,
        statement::{statement::Statement, transform::Transform},
    },
    lexer::{keyword::Keyword, token::Token},
    parser::expr::newline::optional_new_line,
    util::span::{ParserInput, Spanned},
};

pub mod limit;
pub mod skip;
pub mod where_;

pub fn transform_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Transform, Extra<'tokens>> + Clone + 'tokens {
    choice((
        where_::where_parser(stmt.clone()),
        limit::limit_parser(stmt.clone()),
        skip::skip_parser(stmt),
    ))
    .recover_with(skip_then_retry_until(
        any().ignored(),
        one_of(vec![
            Token::Keyword(Keyword::Where),
            Token::Keyword(Keyword::Limit),
            Token::Keyword(Keyword::Skip),
            Token::Keyword(Keyword::Create),
            Token::Keyword(Keyword::Update),
            Token::Keyword(Keyword::Delete),
            Token::Punctuation(';'),
            Token::Newline,
        ])
        .ignored(),
    ))
    // .recover_with(via_parser(invalid_transform_parser()))
}

pub fn unexpected_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Unexpected, Extra<'tokens>> + Clone + 'tokens
{
    let keyword = select! {
        Token::Keyword(k) => k,
    };
    let ident = select! {
        Token::Identifier(s) => s,
    };
    optional_new_line().ignore_then(choice((
        keyword.map(Unexpected::Keyword),
        ident.map(Unexpected::Identifier),
    )))
}

#[derive(Debug, Clone)]
pub enum Unexpected {
    Keyword(Keyword),
    Identifier(String),
}
