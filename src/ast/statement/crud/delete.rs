use crate::{ast::statement::transform::Transform, util::span::Spanned};

pub struct DeleteStatement {
    pub table: Option<Spanned<String>>,
    pub transforms: Vec<Spanned<Transform>>,
}
