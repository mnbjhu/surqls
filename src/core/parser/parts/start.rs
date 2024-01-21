use std::fmt::Display;

use chumsky::{
    extra,
    prelude::Rich,
    primitive::{any, choice, just},
    select, IterParser, Parser,
};

use crate::core::{
    lexer::{Keyword, Token},
    parser::{
        expr::newline::optional_new_line,
        projection::{projection_parser, Projection},
    },
    span::{ParserInput, Span, Spanned},
};

#[derive(Debug, Clone, PartialEq)]
pub enum StatementStart {
    Select(Vec<Spanned<Projection>>),
    Create(Option<Spanned<String>>),
    Delete(Option<Spanned<String>>),
    Update(Option<Spanned<String>>),
}

impl Display for StatementStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementStart::Select(_) => write!(f, "SELECT"),
            StatementStart::Create(_) => write!(f, "CREATE"),
            StatementStart::Delete(_) => write!(f, "DELETE"),
            StatementStart::Update(_) => write!(f, "UPDATE"),
        }
    }
}

pub fn statement_start_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<StatementStart>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> + Clone {
    let ident = select! {
        Token::Identifier(s) => s.to_string(),
    }
    .map_with(|e, s| (e, s.span()));
    let select = just(Token::Keyword(Keyword::Select))
        .ignore_then(optional_new_line())
        .ignore_then(
            projection_parser()
                .separated_by(just(Token::Punctuation(',')).padded_by(optional_new_line()))
                .collect::<Vec<_>>(),
        )
        .map(StatementStart::Select);

    let create = just(Token::Keyword(Keyword::Create))
        .ignore_then(optional_new_line())
        .ignore_then(ident.clone().or_not())
        .map(StatementStart::Create);

    let delete = just(Token::Keyword(Keyword::Delete))
        .ignore_then(optional_new_line())
        .ignore_then(ident.clone().or_not())
        .map(StatementStart::Delete);

    let update = just(Token::Keyword(Keyword::Update))
        .ignore_then(optional_new_line())
        .ignore_then(ident.clone().or_not())
        .map(StatementStart::Update);

    choice((select, create, delete, update)).map_with(|s, span| (s, span.span()))
}
