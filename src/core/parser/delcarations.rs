use std::collections::HashMap;

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

pub struct Object {
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub ty: Type,
}

pub struct ScopedItems {
    pub tables_definitions: HashMap<String, Type>,
    pub scoped_tables: Vec<Object>,
}
