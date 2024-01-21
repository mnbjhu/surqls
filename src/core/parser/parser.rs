use chumsky::{
    extra,
    prelude::Rich,
    primitive::{choice, just},
    IterParser, Parser,
};

use crate::core::{
    lexer::Token,
    span::{ParserInput, Span, Spanned},
};

use super::{
    expr::newline::optional_new_line,
    statement::{crud_statement::crud_statement_parser, statement::Statement},
};

pub type File = Vec<Spanned<Statement>>;

pub fn parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    File,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    crud_statement_parser()
        .map(Statement::Crud)
        .map_with(|s, span| (s, span.span()))
        .separated_by(
            choice((
                just(Token::Newline),
                just(Token::Punctuation(';')).padded_by(optional_new_line()),
            ))
            .ignored(),
        )
        .allow_trailing()
        .collect::<Vec<_>>()
}
