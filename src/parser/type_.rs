use chumsky::{primitive::just, recursive::recursive, select, IterParser, Parser};

use crate::{
    ast::{parser::Extra, type_::Type},
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

pub fn type_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Type>, Extra<'tokens>> + Clone + 'tokens
{
    recursive(|ty| {
        let ident = select!(
            Token::Identifier(ident) => ident,
        )
        .map_with(|i, s| (i.to_string(), s.span()));
        ident
            .clone()
            .then(
                ty.separated_by(just(Token::Punctuation(',')))
                    .collect::<Vec<_>>()
                    .delimited_by(
                        just(Token::Operator("<".to_string())),
                        just(Token::Operator(">".to_string())),
                    )
                    .or_not(),
            )
            .map(|(name, args)| Type {
                name,
                args: args.unwrap_or_default(),
            })
            .map_with(|i, s| (i, s.span()))
    })
}
