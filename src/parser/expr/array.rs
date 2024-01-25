use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    ast::{expr::parser::Expression, parser::Extra},
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::newline::optional_new_line;

pub fn array_parser<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let array = expr
        .separated_by(
            just(Token::Punctuation(','))
                .padded_by(optional_new_line())
                .ignored(),
        )
        .allow_trailing()
        .collect()
        .delimited_by(
            just(Token::Punctuation('[')).then(optional_new_line()),
            optional_new_line().then(just(Token::Punctuation(']'))),
        )
        .map(|v| Expression::Array(v))
        .map_with(|e, s| (e, s.span()));
    array
}
