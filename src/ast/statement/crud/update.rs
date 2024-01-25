use crate::{
    ast::{expr::parser::Expression, statement::transform::Transform},
    util::span::Spanned,
};

pub struct UpdateStatement {
    pub table: Option<Spanned<String>>,
    pub content: Option<Spanned<Expression>>,
    pub transforms: Vec<Spanned<Transform>>,
}
