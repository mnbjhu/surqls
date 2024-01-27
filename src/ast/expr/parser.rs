use crate::{ast::statement::statement::Statement, util::span::Spanned};

use super::{
    access::Access, literal::Literal, object::ObjectEntry, op::BinaryOperator, unary::UnaryOperator,
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
    Variable(String),
    CodeBlock(Vec<Spanned<Statement>>),
    Call {
        name: Vec<Spanned<String>>,
        args: Option<Vec<Spanned<Expression>>>,
    },
}
