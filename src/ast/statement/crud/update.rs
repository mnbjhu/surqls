use crate::{
    ast::{expr::parser::Expression, statement::transform::Transform},
    util::span::Spanned,
};

#[derive(Debug, Clone)]
pub struct UpdateStatement {
    pub table: Option<Spanned<String>>,
    pub content: Option<Spanned<Expression>>,
    pub transforms: Vec<Spanned<Transform>>,
}
