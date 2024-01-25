use std::collections::HashMap;

use super::{field::Field, object::Object, type_::Type};

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
