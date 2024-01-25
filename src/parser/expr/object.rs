use chumsky::{primitive::just, recovery::via_parser, select, IterParser, Parser};

use crate::{
    ast::{
        expr::{object::ObjectEntry, parser::Expression},
        parser::Extra,
    },
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::newline::optional_new_line;

pub fn object_entry<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<ObjectEntry>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let ident = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|i, s| (i.to_string(), s.span()));
    let entry = ident
        .clone()
        .then_ignore(just(Token::Punctuation(':')).padded_by(optional_new_line()))
        .then(expr)
        .map_with(|(k, v), s| {
            (
                ObjectEntry {
                    key: k,
                    value: Some(v),
                },
                s.span(),
            )
        });

    entry.recover_with(via_parser(
        ident
            .then_ignore(
                optional_new_line()
                    .then(just(Token::Punctuation(':')))
                    .or_not(),
            )
            .map_with(|k, s| {
                (
                    ObjectEntry {
                        key: k,
                        value: None,
                    },
                    s.span(),
                )
            }),
    ))
}

pub fn object_parser<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let object = object_entry(expr.clone())
        .separated_by(just(Token::Punctuation(',')).padded_by(optional_new_line()))
        .allow_trailing()
        .collect()
        .delimited_by(
            just(Token::Punctuation('{')).then(optional_new_line()),
            optional_new_line().then(just(Token::Punctuation('}'))),
        )
        .map(|v| Expression::Object(v))
        .map_with(|e, s| (e, s.span()))
        .boxed();
    object
}
