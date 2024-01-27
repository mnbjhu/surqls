use chumsky::{primitive::just, recovery::via_parser, select, IterParser, Parser};

use crate::{
    ast::{
        parser::Extra,
        statement::{
            crud::{create::CreateStatement, select::SelectStatement},
            statement::Statement,
        },
    },
    lexer::{keyword::Keyword, token::Token},
    parser::{expr::newline::optional_new_line, statement::transform::transform_parser},
    util::span::{ParserInput, Spanned},
};

use super::projection::projection_parser;

pub fn select_statement_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, SelectStatement, Extra<'tokens>> + Clone + 'tokens
{
    let ident = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|x, s| (x, s.span()));

    let projections = projection_parser(stmt.clone())
        .separated_by(just(Token::Punctuation(',')))
        .collect::<Vec<_>>();

    let select_part = just(Token::Keyword(Keyword::Select))
        .ignore_then(optional_new_line().ignore_then(projections));

    let from_part = just(Token::Keyword(Keyword::From))
        .ignore_then(optional_new_line().ignore_then(ident))
        .map(|x| Some(x))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::From)).map(|_| None),
        ));
    select_part
        .clone()
        .then(optional_new_line().ignore_then(from_part))
        .recover_with(via_parser(select_part.map(|x| (x, None))))
        .then(
            transform_parser(stmt)
                .map_with(|part, scope| (part, scope.span()))
                .separated_by(optional_new_line())
                .allow_leading()
                .collect::<Vec<_>>(),
        )
        .map(|((projections, from), transforms)| SelectStatement {
            projections,
            from,
            transforms,
        })
}
