use crate::{ast::expr::parser::Expression, util::span::Spanned};

use super::{
    crud::{
        create::CreateStatement, delete::DeleteStatement, select::SelectStatement,
        update::UpdateStatement,
    },
    define::DefineStatement,
    let_::LetStatement,
};

#[derive(Debug, Clone)]
pub enum Statement {
    Select(SelectStatement),
    Create(CreateStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    Return(Spanned<Expression>),
    Define(Spanned<DefineStatement>),
    Let(LetStatement),
    Invalid,
}
