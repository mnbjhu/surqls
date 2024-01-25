use crate::{ast::expr::parser::Expression, util::span::Spanned};

use super::{create::CreateStatement, define::DefineStatement, update::UpdateStatement};

pub enum Statement {
    Create(CreateStatement),
    Update(UpdateStatement),
    Return(Spanned<Expression>),
    Define(Spanned<DefineStatement>),
}
