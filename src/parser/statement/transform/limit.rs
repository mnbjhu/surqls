use chumsky::{primitive::just, recovery::via_parser, Parser};

use crate::{
    ast::{
        parser::Extra,
        statement::{statement::Statement, transform::Transform},
    },
    lexer::{keyword::Keyword, token::Token},
    parser::{
        expr::{newline::optional_new_line, parser::expr_parser},
        statement::statement::statement_parser,
    },
    util::span::{ParserInput, Spanned},
};

pub fn limit_parser<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Transform, Extra<'tokens>> + Clone + 'tokens {
    just(Token::Keyword(Keyword::Limit))
        .ignore_then(optional_new_line().ignore_then(expr_parser(stmt)))
        .map(|x| Transform::Limit(Some(x)))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Limit)).map(|_| Transform::Limit(None)),
        ))
}
