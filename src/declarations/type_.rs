use std::fmt::Display;

use super::object::Object;

#[derive(Clone, Debug)]
pub enum Type {
    Error,
    Null,
    Any,
    Bool,
    Float,
    Int,
    String,
    Array(Box<Type>),
    Option(Box<Type>),
    Object(Object),
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Error => write!(f, "ERROR"),
            Type::Null => write!(f, "null"),
            Type::Any => write!(f, "any"),
            Type::Bool => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Array(t) => write!(f, "array<{}>", t),
            Type::Option(t) => write!(f, "option<{}>", t),
            Type::Object(o) => write!(f, "object"),
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
