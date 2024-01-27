use chumsky::{
    primitive::{any, choice, just},
    recovery::skip_then_retry_until,
    IterParser, Parser,
};

use crate::{
    ast::parser::{Extra, File},
    lexer::token::Token,
    util::span::ParserInput,
};

use super::{
    expr::newline::optional_new_line,
    statement::{statement::statement_parser, statement_start::statement_start_parser},
};

pub fn parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, File, Extra<'tokens>> + Clone + 'tokens {
    statement_parser()
        // .recover_with(via_parser(invalid_statement_parser()))
        .recover_with(skip_then_retry_until(
            any().ignored(),
            statement_start_parser(),
        ))
        .separated_by(
            choice((just(Token::Newline), just(Token::Punctuation(';')))).recover_with(
                skip_then_retry_until(any().ignored(), statement_start_parser()),
            ),
        )
        .allow_trailing()
        .collect::<Vec<_>>()
        .padded_by(optional_new_line())
}
