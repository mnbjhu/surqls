use chumsky::{primitive::just, recovery::via_parser, Parser};

use crate::{
    ast::{parser::Extra, statement::transform::Transform},
    lexer::{keyword::Keyword, token::Token},
    parser::expr::{newline::optional_new_line, parser::expr_parser},
    util::span::ParserInput,
};

pub fn skip_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Transform, Extra<'tokens>> + Clone {
    just(Token::Keyword(Keyword::Skip))
        .ignore_then(optional_new_line().ignore_then(expr_parser()))
        .map(|x| Transform::Skip(Some(x)))
        .recover_with(via_parser(
            just(Token::Keyword(Keyword::Skip)).map(|_| Transform::Skip(None)),
        ))
}
