use chumsky::{extra, prelude::Rich, primitive::just, select, IterParser, Parser};

use crate::core::{
    lexer::Token,
    span::{ParserInput, Span, Spanned},
};

pub struct Type {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<String>>,
}

pub fn type_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<Type>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    let ident = select!(
        Token::Identifier(ident) => ident,
    )
    .map_with(|i, s| (i.to_string(), s.span()));
    ident
        .clone()
        .then(
            ident
                .separated_by(just(Token::Punctuation(',')))
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Operator("<")), just(Token::Operator(">")))
                .or_not(),
        )
        .map(|(name, args)| Type {
            name,
            args: args.unwrap_or_default(),
        })
        .map_with(|i, s| (i, s.span()))
}
