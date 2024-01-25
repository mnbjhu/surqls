use std::collections::HashMap;

use super::{field::Field, object::Object, type_::Type};

#[derive(Clone, Debug)]
pub struct ScopedItems {
    pub table_definitions: HashMap<String, Object>,
    pub scoped_table: Object,
}

impl Default for ScopedItems {
    fn default() -> Self {
        let mut table_definitions = HashMap::new();
        let some = Field {
            name: "some".to_string(),
            ty: Type::String,
        };
        let thing_type = Object { fields: vec![some] };

        table_definitions.insert("thing".to_string(), thing_type);

        let scoped_table = Object {
            fields: vec![Field {
                name: "thing".to_string(),
                ty: Type::String,
            }],
        };

        Self {
            table_definitions,
            scoped_table,
        }
    }
}
