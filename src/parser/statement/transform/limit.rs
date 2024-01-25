use chumsky::{primitive::just, recovery::via_parser, Parser};

use crate::{
    ast::{parser::Extra, statement::transform::Transform},
    lexer::{keyword::Keyword, token::Token},
    parser::expr::{newline::optional_new_line, parser::expr_parser},
    util::span::ParserInput,
};

pub fn limit_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Transform, Extra<'tokens>> + Clone {
    just(Token::Keyword(Keyword::Limit))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|x| Transform::Limit(Some(x)))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Limit)).map(|_| Transform::Limit(None)),
        ))
}
