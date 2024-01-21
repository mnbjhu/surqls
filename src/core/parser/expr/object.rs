use chumsky::{primitive::just, recovery::via_parser, IterParser, Parser};

use crate::core::{
    lexer::Token,
    parser::parser::Extra,
    span::{ParserInput, Spanned},
};

use super::{
    field::{field_parser, Field},
    newline::optional_new_line,
    parser::Expression,
};

pub fn object_entry<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<ObjectEntry>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let entry = field_parser()
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
        field_parser()
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

#[derive(Debug, Clone)]
pub struct ObjectEntry {
    pub key: Spanned<Field>,
    pub value: Option<Spanned<Expression>>,
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
