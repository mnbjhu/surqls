use crate::util::span::Spanned;

pub struct Type {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Type>>,
}
