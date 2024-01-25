use crate::util::span::Spanned;

use self::{field::DefineField, table::DefineTable};

pub mod field;
pub mod table;

pub enum DefineStatement {
    Table(Spanned<DefineTable>),
    Field(Spanned<DefineField>),
}
