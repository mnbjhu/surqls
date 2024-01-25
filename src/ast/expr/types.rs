use crate::declarations::{scoped_item::ScopedItems, type_::Type};

use super::{literal::Literal, parser::Expression};

pub trait Typed {
    fn get_type(&self, scope: &ScopedItems) -> Type;
}

impl Typed for Expression {
    fn get_type(&self, scope: &ScopedItems) -> Type {
        match self {
            Expression::Literal(literal) => match literal {
                Literal::String(_) => Type::String,
                Literal::Int(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::Bool(_) => Type::Bool,
                Literal::Null => Type::Null,
            },
            Expression::Identifier(name) => {
                if let Some(field) = scope.scoped_table.get_field(name) {
                    field.ty.clone()
                } else {
                    Type::Error
                }
            }
            _ => Type::Error,
        }
    }
}
