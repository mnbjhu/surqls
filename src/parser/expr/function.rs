use chumsky::{primitive::just, select, IterParser, Parser};

use crate::{
    ast::{expr::parser::Expression, parser::Extra},
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::newline::optional_new_line;

pub fn function_parser<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let ident = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|s, span| (s, span.span()));
    let function_sep = just(Token::Punctuation(':')).then(just(Token::Punctuation(':')));
    let part = ident.clone().then_ignore(function_sep);

    let name = part
        .clone()
        .then(
            part.repeated()
                .collect::<Vec<_>>()
                .then(ident.or_not())
                .map(|(mut v, last)| {
                    if let Some(last) = last {
                        v.push(last);
                    }
                    v
                }),
        )
        .map(|(first, rest)| {
            let mut v = vec![first];
            v.extend(rest);
            v
        });

    let args = expr
        .separated_by(just(Token::Punctuation(',')).padded_by(optional_new_line()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(Token::Punctuation('(')).then(optional_new_line()),
            optional_new_line().then(just(Token::Punctuation(')'))),
        );

    name.then(args.or_not())
        .map_with(|(name, args), s| (Expression::Call { name, args }, s.span()))
}
