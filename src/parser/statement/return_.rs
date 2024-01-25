use chumsky::{primitive::just, recovery::via_parser, Parser};

use crate::{
    ast::{
        expr::{literal::Literal, parser::Expression},
        parser::Extra,
    },
    lexer::{keyword::Keyword, token::Token},
    parser::expr::{newline::optional_new_line, parser::expr_parser},
    util::span::{ParserInput, Spanned},
};

pub fn return_statement_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>> + Clone {
    let return_part = just(Token::Keyword(Keyword::Return))
        .ignore_then(optional_new_line().ignore_then(expr_parser()));

    let missing_value = just(Token::Keyword(Keyword::Return))
        .map(|_| Expression::Literal(Literal::Null))
        .map_with(|e, s| (e, s.span()));

    return_part.recover_with(via_parser(missing_value))
}
