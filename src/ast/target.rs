use crate::util::span::Spanned;

pub enum Target {
    Table(Spanned<String>),
    Record(Spanned<String>, Spanned<String>),
}

