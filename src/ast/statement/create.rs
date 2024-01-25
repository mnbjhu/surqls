use std::fmt::Display;

use crate::{ast::expr::parser::Expression, util::span::Spanned};

pub struct CreateStatement {
    pub table: Option<Spanned<String>>,
    pub content: Option<Spanned<Expression>>,
    pub transforms: Vec<Spanned<Transform>>,
}

pub enum Transform {
    Where(Option<Spanned<Expression>>),
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transform::Where(_) => write!(f, "where"),
        }
    }
}
