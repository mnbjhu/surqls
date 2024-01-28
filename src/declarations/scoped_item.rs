use std::collections::HashMap;

use super::{
    functions::{get_functions, Function},
    object::Object,
    type_::Type,
};

#[derive(Clone, Debug)]
pub struct ScopedItems {
    pub table_definitions: HashMap<String, Object>,
    pub scoped_table: Object,
    pub variables: HashMap<String, Type>,
    pub functions: HashMap<String, Function>,
}

impl Default for ScopedItems {
    fn default() -> Self {
        let table_definitions = HashMap::new();
        let scoped_table = Object { fields: vec![] };
        let variables = HashMap::new();
        let mut functions = get_functions();
        Self {
            table_definitions,
            scoped_table,
            variables,
            functions,
        }
    }
}
