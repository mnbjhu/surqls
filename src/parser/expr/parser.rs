use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, Parser,
};

use crate::{
    ast::{expr::parser::Expression, parser::Extra},
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::{
    access::access_parser, array::array_parser, literal::literal_parser, object::object_parser,
    op::op_parser,
};

pub fn expr_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>> + Clone {
    recursive(|expr| {
        let literal = literal_parser()
            .map(Expression::Literal)
            .map_with(|e, s| (e, s.span()));
        let ident = select! {
            Token::Identifier(s) => s.to_string(),
        }
        .map(Expression::Identifier)
        .map_with(|e, s| (e, s.span()));
        let bracketed = expr.clone().delimited_by(
            just(Token::Punctuation('(')).ignored(),
            just(Token::Punctuation(')')).ignored(),
        );
        let atom = choice((literal, ident, bracketed));
        let access = access_parser(atom, expr.clone());

        op_parser(access.clone())
            .or(access)
            .or(array_parser(expr.clone()))
            .or(object_parser(expr))
    })
    .labelled("expression")
}
