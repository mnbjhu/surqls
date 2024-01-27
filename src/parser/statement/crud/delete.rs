use chumsky::{primitive::just, recovery::via_parser, select, IterParser, Parser};

use crate::{
    ast::{
        parser::Extra,
        statement::{crud::delete::DeleteStatement, statement::Statement},
    },
    lexer::{keyword::Keyword, token::Token},
    parser::{expr::newline::optional_new_line, statement::transform::transform_parser},
    util::span::{ParserInput, Spanned},
};

pub fn delete_statement_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, DeleteStatement, Extra<'tokens>> + Clone + 'tokens
{
    let ident = select! {
        Token::Identifier(s) => s,
    }
    .map_with(|x, s| (x, s.span()));
    let delete_part = just(Token::Keyword(Keyword::Delete))
        .ignore_then(optional_new_line().ignore_then(ident))
        .map(|x| Some(x))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Create)).map(|_| None),
        ));

    delete_part
        .clone()
        .then_ignore(optional_new_line())
        .then(
            transform_parser(stmt)
                .map_with(|part, scope| (part, scope.span()))
                .separated_by(optional_new_line())
                .allow_leading()
                .collect::<Vec<_>>(),
        )
        .map(|(table, transforms)| DeleteStatement { table, transforms })
}
