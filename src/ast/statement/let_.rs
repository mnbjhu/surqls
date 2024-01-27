use crate::{ast::expr::parser::Expression, util::span::Spanned};

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Option<Spanned<String>>,
    pub value: Option<Spanned<Expression>>,
}
