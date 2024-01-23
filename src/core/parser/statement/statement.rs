use crate::core::{parser::expr::parser::Expression, span::Spanned};

use super::{create_statement::CreateStatement, define::DefineStatement};

pub enum Statement {
    Create(CreateStatement),
    Return(Spanned<Expression>),
    Define(Spanned<DefineStatement>),
    Invalid,
}
