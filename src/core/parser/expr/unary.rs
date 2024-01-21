use std::fmt::Display;

use crate::core::span::Spanned;

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
    Invalid { recovered: Spanned<String> },
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOperator::Not => "!",
            UnaryOperator::Negate => "-",
            UnaryOperator::Invalid { recovered: _ } => return write!(f, "Invalid unary operator"),
        };
        write!(f, "{}", s)
    }
}
