use chumsky::{
    extra,
    prelude::Rich,
    primitive::{any, choice, end, just},
    recovery::skip_then_retry_until,
    IterParser, Parser,
};

use crate::core::{
    lexer::{Keyword, Token},
    span::{ParserInput, Span, Spanned},
};

use super::{
    delcarations::ScopedItems,
    expr::newline::optional_new_line,
    statement::{
        create_statement::create_statement_parser, return_::return_statement_parser,
        statement::Statement,
    },
};

pub type File = Vec<Spanned<Statement>>;
pub type Extra<'tokens> = extra::Full<Rich<'tokens, Token, Span>, ScopedItems, ()>;

pub fn parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, File, Extra<'tokens>> + Clone {
    let statement = choice((
        create_statement_parser().map(Statement::Create),
        return_statement_parser().map(Statement::Return),
    ));
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
            )),
        ))
        .separated_by(choice((
            just(Token::Newline),
            just(Token::Punctuation(';')),
        )))
        .allow_trailing()
        .collect::<Vec<_>>()
        .padded_by(optional_new_line())
}
