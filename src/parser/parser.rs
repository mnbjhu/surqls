use chumsky::{
    primitive::{any, choice, end, just},
    recovery::{skip_then_retry_until, via_parser},
    IterParser, Parser,
};

use crate::{
    ast::{
        parser::{Extra, File},
        statement::statement::Statement,
    },
    lexer::{keyword::Keyword, token::Token},
    util::span::ParserInput,
};

use super::{
    expr::newline::optional_new_line,
    statement::{
        crud::{
            create::create_statement_parser, delete::delete_statement_parser,
            select::select_statement_parser, update::update_statement_parser,
        },
        define::define_statement_parser,
        invalid::invalid_statement_parser,
        return_::return_statement_parser,
    },
};

pub fn parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, File, Extra<'tokens>> + Clone {
    let statement = choice((
        create_statement_parser().map(Statement::Create),
        update_statement_parser().map(Statement::Update),
        delete_statement_parser().map(Statement::Delete),
        return_statement_parser().map(Statement::Return),
        define_statement_parser().map(Statement::Define),
        select_statement_parser().map(Statement::Select),
    ))
    .recover_with(via_parser(invalid_statement_parser()));
    statement
        .map_with(|s, span| (s, span.span()))
        // .recover_with(via_parser(invalid_statement_parser()))
        .recover_with(skip_then_retry_until(
            any().ignored(),
            choice((
                end().ignored(),
                just(Token::Keyword(Keyword::Create)).ignored(),
                just(Token::Keyword(Keyword::Update)).ignored(),
                just(Token::Keyword(Keyword::Delete)).ignored(),
                just(Token::Keyword(Keyword::Select)).ignored(),
                just(Token::Keyword(Keyword::Define)).ignored(),
            )),
        ))
        .separated_by(
            choice((just(Token::Newline), just(Token::Punctuation(';')))).recover_with(
                skip_then_retry_until(
                    any().ignored(),
                    choice((
                        end().ignored(),
                        just(Token::Keyword(Keyword::Create)).ignored(),
                        just(Token::Keyword(Keyword::Update)).ignored(),
                        just(Token::Keyword(Keyword::Delete)).ignored(),
                        just(Token::Keyword(Keyword::Select)).ignored(),
                        just(Token::Keyword(Keyword::Define)).ignored(),
                    )),
                ),
            ),
        )
        .allow_trailing()
        .collect::<Vec<_>>()
        .padded_by(optional_new_line())
}
