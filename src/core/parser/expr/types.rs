use crate::core::parser::delcarations::{Object, Type};

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

impl Type {
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Error, _) => true,
            (_, Type::Error) => true,
            (Type::Null, Type::Null) => true,
            (Type::Any, _) => true,
            (_, Type::Any) => false,
            (Type::Bool, Type::Bool) => true,
            (Type::Float, Type::Float) => true,
            (Type::Int, Type::Int) => true,
            (Type::String, Type::String) => true,
            (Type::Array(a), Type::Array(b)) => a.is_assignable_to(b),
            (Type::Option(a), Type::Option(b)) => a.is_assignable_to(b),
            (Type::Object(a), Type::Object(b)) => a.is_assignable_to(b),
            _ => false,
        }
    }
}

impl Object {
    pub fn is_assignable_to(&self, other: &Object) -> bool {
        for field in &self.fields {
            if let Some(other_field) = other.get_field(&field.name) {
                if !field.ty.is_assignable_to(&other_field.ty) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
