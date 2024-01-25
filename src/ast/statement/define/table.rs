use crate::util::span::Spanned;

pub enum Permission {
    Full,
    None,
}

pub struct DefineTable {
    pub name: Spanned<String>,
    pub permission: Option<Spanned<Permission>>,
}
