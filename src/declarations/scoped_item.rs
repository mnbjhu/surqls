use std::collections::HashMap;

use super::{object::Object, type_::Type};

#[derive(Clone, Debug)]
pub struct ScopedItems {
    pub table_definitions: HashMap<String, Object>,
    pub scoped_table: Object,
    pub variables: HashMap<String, Type>,
}

impl Default for ScopedItems {
    fn default() -> Self {
        let table_definitions = HashMap::new();
        let scoped_table = Object { fields: vec![] };
        let variables = HashMap::new();
        Self {
            table_definitions,
            scoped_table,
            variables,
        }
    }
}
