use chumsky::{
    primitive::{choice, just},
    select, Parser,
};

use crate::core::{
    lexer::Token,
    parser::parser::Extra,
    span::{ParserInput, Spanned},
};

use super::{newline::optional_new_line, parser::Expression};

#[derive(Clone, Debug)]
pub enum Access {
    Property(String),
    Index(Spanned<Expression>),
}

pub fn access_parser<'tokens, 'src: 'tokens>(
    atom: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let index = expr
        .clone()
        .delimited_by(
            just(Token::Punctuation('['))
                .ignored()
                .padded_by(optional_new_line()),
            optional_new_line().then_ignore(just(Token::Punctuation(']'))),
        )
        .map(Access::Index)
        .boxed();

    let field = just(Token::Punctuation('.'))
        .padded_by(optional_new_line())
        .ignore_then(select! {
            Token::Identifier(s) => s,
        })
        .labelled("field")
        .map(|s| Access::Property(s.to_string()))
        .boxed();

    let access = atom
        .clone()
        .foldl_with(choice((index, field)).repeated(), |a, acc, s| match acc {
            Access::Index(index) => (
                Expression::Access {
                    expr: Box::new(a),
                    access: (Box::new(Access::Index(index)), s.span()),
                },
                s.span(),
            ),
            Access::Property(field) => (
                Expression::Access {
                    expr: Box::new(a),
                    access: (Box::new(Access::Property(field)), s.span()),
                },
                s.span(),
            ),
        })
        .boxed();

    access
}
