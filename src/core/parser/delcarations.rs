use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug)]
pub enum Type {
    Any,
    Bool,
    Float,
    Int,
    String,
    Array(Box<Type>),
    Option(Box<Type>),
    Object(Object),
}

#[derive(Clone, Debug)]
pub struct Object {
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

#[derive(Clone, Default, Debug)]
pub struct ScopedItems {
    pub table_definitions: HashMap<String, Type>,
    pub scoped_table: HashMap<String, Type>,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
