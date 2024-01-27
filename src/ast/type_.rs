use crate::util::span::Spanned;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Type>>,
}
