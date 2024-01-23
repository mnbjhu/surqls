use crate::core::{
    lexer::Token,
    parser::parser::Extra,
    span::{ParserInput, Spanned},
};
use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, Parser,
};

use super::{
    access::{access_parser, Access},
    array::array_parser,
    literal::{literal_parser, Literal},
    object::{object_parser, ObjectEntry},
    op::{op_parser, BinaryOperator},
    unary::UnaryOperator,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary {
        left: Box<Spanned<Expression>>,
        op: Spanned<BinaryOperator>,
        right: Box<Spanned<Expression>>,
    },
    Unary {
        op: Spanned<UnaryOperator>,
        expr: Box<Spanned<Expression>>,
    },
    Access {
        expr: Box<Spanned<Expression>>,
        access: Spanned<Box<Access>>,
    },
    Array(Vec<Spanned<Expression>>),
    Object(Vec<Spanned<ObjectEntry>>),
}

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
