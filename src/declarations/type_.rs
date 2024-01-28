use std::fmt::Display;

use super::object::Object;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Error,
    Null,
    Any,
    Bool,
    Float,
    Int,
    Decimal,
    Number,
    String,
    DateTime,
    Duration,
    Array(Box<Type>),
    Option(Box<Type>),
    Record(String),
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
            Type::Record(s) => write!(f, "record<{}>", s),
            Type::Object(_) => write!(f, "object"),
            Type::DateTime => write!(f, "datetime"),
            Type::Duration => write!(f, "duration"),
            Type::Decimal => write!(f, "decimal"),
            Type::Number => write!(f, "number"),
        }
    }
}

impl Type {
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        if self == other {
            return true;
        }
        match (self, other) {
            (Type::Option(a), Type::Option(b)) => a.is_assignable_to(b),
            (Type::Option(_), Type::Null) => true,
            (Type::Option(a), b) => a.is_assignable_to(b),
            (Type::Error, _) => true,
            (_, Type::Error) => true,
            (Type::Any, _) => true,
            (_, Type::Any) => false,
            (Type::Array(a), Type::Array(b)) => a.is_assignable_to(b),
            (Type::Object(a), Type::Object(b)) => a.is_assignable_to(b),
            (Type::Record(a), Type::Record(b)) => a == b,
            (Type::Number, Type::Int) => true,
            (Type::Number, Type::Float) => true,
            (Type::Number, Type::Decimal) => true,
            (Type::Decimal, Type::Int) => true,
            (Type::Decimal, Type::Float) => true,
            (Type::Float, Type::Int) => true,
            _ => false,
        }
    }

    pub fn get_shared_super_type(&self, other: &Type) -> Type {
        if self == other {
            return self.clone();
        }
        match (self, other) {
            (Type::Option(a), Type::Option(b)) => {
                Type::Option(Box::new(a.get_shared_super_type(b)))
            }
            (Type::Option(a), Type::Null) => Type::Option(a.clone()),
            (Type::Null, Type::Option(b)) => Type::Option(b.clone()),
            (Type::Null, b) => Type::Option(Box::new(b.clone())),
            (a, Type::Null) => Type::Option(Box::new(a.clone())),
            (Type::Error, _) => Type::Error,
            (_, Type::Error) => Type::Error,
            (Type::Any, _) => Type::Any,
            (_, Type::Any) => Type::Any,
            (Type::Array(a), Type::Array(b)) => Type::Array(Box::new(a.get_shared_super_type(b))),
            (Type::Object(a), Type::Object(b)) => {
                if a == b {
                    Type::Object(a.clone())
                } else {
                    Type::Error
                }
            }
            (Type::Record(a), Type::Record(b)) => {
                if a == b {
                    Type::Record(a.clone())
                } else {
                    Type::Error
                }
            }
            (Type::Number, Type::Int) => Type::Number,
            (Type::Number, Type::Float) => Type::Number,
            (Type::Number, Type::Decimal) => Type::Number,
            (Type::Decimal, Type::Int) => Type::Decimal,
            (Type::Decimal, Type::Float) => Type::Decimal,
            (Type::Float, Type::Int) => Type::Float,

            (Type::Int, Type::Number) => Type::Number,
            (Type::Float, Type::Number) => Type::Number,
            (Type::Decimal, Type::Number) => Type::Number,
            (Type::Int, Type::Decimal) => Type::Decimal,
            (Type::Float, Type::Decimal) => Type::Decimal,
            (Type::Int, Type::Float) => Type::Float,

            _ => Type::Any,
        }
    }
}
