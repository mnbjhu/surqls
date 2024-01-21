use chumsky::{
    extra,
    prelude::Rich,
    primitive::{any, choice, end, just, none_of, one_of},
    recovery::{skip_then_retry_until, via_parser},
    IterParser, Parser,
};
use ropey::Rope;
use tower_lsp::lsp_types::Diagnostic;

use crate::core::{
    lexer::{Keyword, Token},
    span::{ParserInput, Span, Spanned},
};

use super::{
    delcarations::ScopedItems,
    diagnostic::HasDiagnostic,
    expr::newline::optional_new_line,
    statement::{
        create_statement::create_statement_parser, crud_statement::crud_statement_parser,
        statement::Statement,
    },
};

pub type File = Vec<Spanned<Statement>>;
pub type Extra<'tokens> = extra::Full<Rich<'tokens, Token, Span>, ScopedItems, ()>;

pub fn parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, File, Extra<'tokens>> + Clone {
    let statement = choice((create_statement_parser().map(Statement::Create),));
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

impl HasDiagnostic for File {
    fn diagnostics(&self, rope: &Rope) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for statement in self {
            diagnostics.extend(statement.diagnostics(rope));
        }
        diagnostics
    }
}
pub fn invalid_statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>> + Clone {
    let other = none_of(vec![
        Token::Keyword(Keyword::Create),
        Token::Keyword(Keyword::Select),
        Token::Keyword(Keyword::Update),
        Token::Keyword(Keyword::Delete),
        Token::Punctuation(';'),
    ]);
    let invalid = other.clone().then_ignore(other).map(|_| Statement::Invalid);
    optional_new_line().ignore_then(invalid.map_with(|s, span| (s, span.span())))
}
