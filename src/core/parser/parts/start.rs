use std::fmt::Display;

use chumsky::{
    primitive::{choice, just},
    IterParser, Parser,
};

use crate::core::{
    lexer::{Keyword, Token},
    parser::{
        expr::newline::optional_new_line,
        parser::Extra,
        projection::{projection_parser, Projection},
        table_name::{table_name_parser, TableName},
    },
    span::{ParserInput, Spanned},
};

#[derive(Debug, Clone)]
pub enum StatementStart {
    Select(Vec<Spanned<Projection>>),
    Delete(Option<Spanned<TableName>>),
    Update(Option<Spanned<TableName>>),
}

impl Display for StatementStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementStart::Select(_) => write!(f, "SELECT",),
            StatementStart::Delete(target) => match target {
                Some(target) => write!(f, "DELETE {:?}", target.0),
                None => write!(f, "DELETE"),
            },
            StatementStart::Update(target) => match target {
                Some(target) => write!(f, "UPDATE {:?}", target.0),
                None => write!(f, "UPDATE"),
            },
        }
    }
}

pub fn statement_start_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<StatementStart>, Extra<'tokens>> + Clone
{
    let select = just(Token::Keyword(Keyword::Select))
        .ignore_then(
            optional_new_line().ignore_then(
                projection_parser()
                    .separated_by(just(Token::Punctuation(',')).padded_by(optional_new_line()))
                    .collect::<Vec<_>>(),
            ),
        )
        .map(StatementStart::Select);

    let delete = just(Token::Keyword(Keyword::Delete))
        .ignore_then(
            optional_new_line()
                .ignore_then(table_name_parser())
                .or_not(),
        )
        .map(StatementStart::Delete);

    let update = just(Token::Keyword(Keyword::Update))
        .ignore_then(
            optional_new_line()
                .ignore_then(table_name_parser())
                .or_not(),
        )
        .map(StatementStart::Update);

    choice((select, delete, update)).map_with(|s, span| (s, span.span()))
}
