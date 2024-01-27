use chumsky::{primitive::choice, recovery::via_parser, recursive::recursive, Parser};

use crate::{
    ast::{parser::Extra, statement::statement::Statement},
    util::span::{ParserInput, Spanned},
};

use super::{
    crud::{
        create::create_statement_parser, delete::delete_statement_parser,
        select::select_statement_parser, update::update_statement_parser,
    },
    define::define_statement_parser,
    invalid::invalid_statement_parser,
    let_::let_statement_parser,
    return_::return_statement_parser,
};

pub fn statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>> + Clone + 'tokens
{
    recursive(|stmt| {
        let statement = choice((
            create_statement_parser(stmt.clone()).map(Statement::Create),
            update_statement_parser(stmt.clone()).map(Statement::Update),
            delete_statement_parser(stmt.clone()).map(Statement::Delete),
            return_statement_parser(stmt.clone()).map(Statement::Return),
            define_statement_parser(stmt.clone()).map(Statement::Define),
            select_statement_parser(stmt.clone()).map(Statement::Select),
            let_statement_parser(stmt).map(Statement::Let),
        ))
        .recover_with(via_parser(invalid_statement_parser()));
        statement.map_with(|s, span| (s, span.span()))
        // .recover_with(via_parser(invalid_statement_parser()))
        // .recover_with(skip_then_retry_until(
        //     any().ignored(),
        //     statement_start_parser(),
        // ))
    })
}
