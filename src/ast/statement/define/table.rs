use crate::util::span::Spanned;

#[derive(Debug, Clone)]
pub enum Permission {
    Full,
    None,
}

#[derive(Debug, Clone)]
pub struct DefineTable {
    pub name: Spanned<String>,
    pub permission: Option<Spanned<Permission>>,
}
