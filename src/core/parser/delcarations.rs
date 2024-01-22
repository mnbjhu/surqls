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

impl Object {
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct ScopedItems {
    pub table_definitions: HashMap<String, Type>,
    pub scoped_table: HashMap<String, Type>,
}

impl Default for ScopedItems {
    fn default() -> Self {
        let mut table_definitions = HashMap::new();
        let some = Field {
            name: "some".to_string(),
            ty: Type::String,
        };
        let thing_type = Type::Object(Object { fields: vec![some] });

        table_definitions.insert("thing".to_string(), thing_type);

        let mut scoped_table = HashMap::new();
        scoped_table.insert("some".to_string(), Type::String);
        scoped_table.insert("other".to_string(), Type::Int);

        Self {
            table_definitions,
            scoped_table,
        }
    }
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
