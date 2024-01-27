use chumsky::{
    primitive::{choice, just},
    IterParser, Parser,
};

use crate::{
    ast::{expr::parser::Expression, parser::Extra, statement::statement::Statement},
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::newline::optional_new_line;

pub fn code_block_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    stmt
        // .recover_with(via_parser(invalid_statement_parser()))
        .separated_by(choice((
            just(Token::Newline),
            just(Token::Punctuation(';')),
        )))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(Token::Punctuation('{')).then(optional_new_line()),
            just(Token::Punctuation('}')),
        )
        .map(Expression::CodeBlock)
        .map_with(|e, s| (e, s.span()))
}
