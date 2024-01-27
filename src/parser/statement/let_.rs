use chumsky::{primitive::just, recovery::via_parser, select, Parser};

use crate::{
    ast::{
        parser::Extra,
        statement::{let_::LetStatement, statement::Statement},
    },
    lexer::{keyword::Keyword, token::Token},
    parser::expr::{newline::optional_new_line, parser::expr_parser},
    util::span::{ParserInput, Spanned},
};

pub fn let_statement_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, LetStatement, Extra<'tokens>> + Clone + 'tokens
{
    let var = select! {
        Token::Variable(var) => var,
    }
    .map_with(|v, s| (v, s.span()));
    let let_part =
        just(Token::Keyword(Keyword::Let)).ignore_then(optional_new_line().ignore_then(var));

    let_part
        .clone()
        .then_ignore(just(Token::Operator("=".to_string())))
        .then_ignore(optional_new_line())
        .then(expr_parser(stmt))
        .map(|(var, expr)| LetStatement {
            name: Some(var),
            value: Some(expr),
        })
        .recover_with(via_parser(
            let_part
                .clone()
                .then_ignore(just(Token::Operator("=".to_string())))
                .map(|var| LetStatement {
                    name: Some(var),
                    value: None,
                }),
        ))
        .recover_with(via_parser(let_part.map(|var| LetStatement {
            name: Some(var),
            value: None,
        })))
        .recover_with(via_parser(just(Token::Keyword(Keyword::Let)).map(|_| {
            LetStatement {
                name: None,
                value: None,
            }
        })))
}
