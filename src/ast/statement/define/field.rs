use crate::{ast::type_::Type, util::span::Spanned};

use super::table::Permission;

#[derive(Debug, Clone)]
pub struct DefineField {
    pub name: Spanned<String>,
    pub parents: Vec<Spanned<String>>,
    pub table_name: Spanned<String>,
    pub type_: Spanned<Type>,
    pub permission: Option<Spanned<Permission>>,
}
