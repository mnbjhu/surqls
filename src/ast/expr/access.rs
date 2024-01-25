use crate::util::span::Spanned;

use super::parser::Expression;

#[derive(Clone, Debug)]
pub enum Access {
    Property(String),
    Index(Spanned<Expression>),
}
