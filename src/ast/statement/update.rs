use crate::{ast::expr::parser::Expression, util::span::Spanned};

use super::create::Transform;

pub struct UpdateStatement {
    pub table: Option<Spanned<String>>,
    pub content: Option<Spanned<Expression>>,
    pub transforms: Vec<Spanned<Transform>>,
}
