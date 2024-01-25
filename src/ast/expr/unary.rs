use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnaryOperator::Not => "!",
            UnaryOperator::Negate => "-",
        };
        write!(f, "{}", s)
    }
}
