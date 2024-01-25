use crate::util::span::Spanned;

use super::parser::Expression;

#[derive(Debug, Clone)]
pub struct ObjectEntry {
    pub key: Spanned<String>,
    pub value: Option<Spanned<Expression>>,
}
