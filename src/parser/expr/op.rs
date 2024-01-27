use chumsky::{
    primitive::{choice, just},
    Parser,
};

use crate::{
    ast::{
        expr::{op::BinaryOperator, parser::Expression, unary::UnaryOperator},
        parser::Extra,
    },
    lexer::token::Token,
    util::span::{ParserInput, Spanned},
};

use super::newline::optional_new_line;

pub fn op_parser<'tokens, 'src: 'tokens>(
    atom: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let unary_op = choice((
        just(Token::Operator("-".to_string())).map(|_| UnaryOperator::Negate),
        just(Token::Operator("!".to_string())).map(|_| UnaryOperator::Not),
    ));

    let unary = unary_op
        .map_with(|op, s| (op, s.span()))
        .clone()
        .then(atom.clone())
        .map_with(|(op, expr), s| {
            (
                Expression::Unary {
                    op,
                    expr: Box::new(expr),
                },
                s.span(),
            )
        })
        .or(atom.clone())
        .boxed();

    let mul = unary
        .clone()
        .foldl_with(
            choice((
                just(Token::Operator("*".to_string())).map(|_| BinaryOperator::Multiply),
                just(Token::Operator("/".to_string())).map(|_| BinaryOperator::Divide),
            ))
            .map_with(|op, s| (op, s.span()))
            .padded_by(optional_new_line())
            .then(unary)
            .repeated(),
            |a, (op, b), s| {
                (
                    Expression::Binary {
                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    },
                    s.span(),
                )
            },
        )
        .boxed();

    let add = mul
        .clone()
        .foldl_with(
            choice((
                just(Token::Operator("+".to_string())).map(|_| BinaryOperator::Add),
                just(Token::Operator("-".to_string())).map(|_| BinaryOperator::Subtract),
                just(Token::Operator("%".to_string())).map(|_| BinaryOperator::Modulo),
            ))
            .map_with(|op, s| (op, s.span()))
            .padded_by(optional_new_line())
            .then(mul)
            .repeated(),
            |a, (op, b), s| {
                (
                    Expression::Binary {
                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    },
                    s.span(),
                )
            },
        )
        .boxed();

    let cmp = add
        .clone()
        .foldl_with(
            choice((
                just(Token::Operator("==".to_string())).map(|_| BinaryOperator::Equals),
                just(Token::Operator("!=".to_string())).map(|_| BinaryOperator::NotEquals),
                just(Token::Operator(">".to_string())).map(|_| BinaryOperator::GreaterThan),
                just(Token::Operator("<".to_string())).map(|_| BinaryOperator::LessThan),
                just(Token::Operator(">=".to_string())).map(|_| BinaryOperator::GreaterThanOrEqual),
                just(Token::Operator("<=".to_string())).map(|_| BinaryOperator::LessThanOrEqual),
            ))
            .map_with(|op, s| (op, s.span()))
            .padded_by(optional_new_line())
            .then(add)
            .repeated(),
            |a, (op, b), s| {
                (
                    Expression::Binary {
                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    },
                    s.span(),
                )
            },
        )
        .boxed();

    let and = cmp
        .clone()
        .foldl_with(
            just(Token::Operator("&&".to_string()))
                .padded_by(optional_new_line())
                .map_with(|_, s| (BinaryOperator::And, s.span()))
                .then(cmp)
                .repeated(),
            |a, (op, b), s| {
                (
                    Expression::Binary {
                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    },
                    s.span(),
                )
            },
        )
        .boxed();

    let or = and
        .clone()
        .foldl_with(
            just(Token::Operator("||".to_string()))
                .padded_by(optional_new_line())
                .map_with(|_, s| (BinaryOperator::Or, s.span()))
                .then(and)
                .repeated(),
            |a, (op, b), s| {
                (
                    Expression::Binary {
                        left: Box::new(a),
                        op,
                        right: Box::new(b),
                    },
                    s.span(),
                )
            },
        )
        .boxed();
    or
}
