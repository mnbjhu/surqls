use crate::util::span::Spanned;

use super::expr::parser::Expression;

#[derive(Debug, Clone)]
pub struct Projection {
    pub expr: Spanned<Expression>,
    pub alias: Option<Spanned<String>>,
}
