use crate::declarations::type_::Type;

use super::{literal::Literal, parser::Expression};

pub trait Typed {
    fn get_type(&self) -> Type;
}

impl Typed for Expression {
    fn get_type(&self) -> Type {
        match self {
            Expression::Literal(literal) => match literal {
                Literal::String(_) => Type::String,
                Literal::Int(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::Bool(_) => Type::Bool,
                Literal::Null => Type::Null,
            },
            _ => Type::Error,
        }
    }
}
