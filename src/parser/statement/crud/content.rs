use chumsky::{primitive::just, recovery::via_parser, Parser};

use crate::{
    ast::{expr::parser::Expression, parser::Extra, statement::statement::Statement},
    lexer::{keyword::Keyword, token::Token},
    parser::{
        expr::{newline::optional_new_line, parser::expr_parser},
        statement::statement::statement_parser,
    },
    util::span::{ParserInput, Spanned},
};

pub fn content_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Option<Spanned<Expression>>, Extra<'tokens>>
       + Clone
       + 'tokens {
    just(Token::Keyword(Keyword::Content))
        .ignore_then(optional_new_line().ignore_then(expr_parser(stmt)))
        .map(|x| Some(x))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Content)).map(|_| None),
        ))
}
