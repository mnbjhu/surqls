use crate::util::span::Spanned;

use self::{field::DefineField, table::DefineTable};

pub mod field;
pub mod table;

#[derive(Debug, Clone)]
pub enum DefineStatement {
    Table(Spanned<DefineTable>),
    Field(Spanned<DefineField>),
}
